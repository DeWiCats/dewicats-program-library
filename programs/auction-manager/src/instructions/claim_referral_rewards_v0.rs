use crate::{auction_manager_seeds, error::ErrorCode, state::*};
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::Transfer;
use anchor_spl::{
  associated_token::AssociatedToken,
  token::{self, Token, TokenAccount},
};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct ClaimReferralRewardsArgsV0 {
  recipient_wallet: Pubkey,
}

#[derive(Accounts)]
pub struct ClaimReferralRewardsV0<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,
  pub auction_manager: Box<Account<'info, AuctionManagerV0>>,
  #[account(
        mut,
        has_one = token_mint,
      )]
  pub listing: Box<Account<'info, ListingV0>>,
  pub token_mint: Box<Account<'info, Mint>>,
  #[account(
        mut,
        has_one = listing
      )]
  pub bid_reciept: Box<Account<'info, BidRecieptV0>>,
  #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = token_mint,
        associated_token::authority = auction_manager
    )]
  pub token_escrow: Box<Account<'info, TokenAccount>>,

  #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = token_mint,
        associated_token::authority = payer
    )]
  pub token_source: Box<Account<'info, TokenAccount>>,
  #[account(mut)]
  pub referral_recipient: Box<Account<'info, ReferralRecipientV0>>,
  pub associated_token_program: Program<'info, AssociatedToken>,
  pub system_program: Program<'info, System>,
  pub token_program: Program<'info, Token>,
  pub rent: Sysvar<'info, Rent>,
}

impl<'info> ClaimReferralRewardsV0<'info> {
  fn transfer_escrow_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
    let cpi_accounts = Transfer {
      from: self.token_escrow.to_account_info(),
      to: self.token_source.to_account_info(),
      authority: self.auction_manager.to_account_info(),
    };
    CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
  }
}

pub fn handler(
  ctx: Context<ClaimReferralRewardsV0>,
  _args: ClaimReferralRewardsArgsV0,
) -> Result<()> {
  if ctx.accounts.listing.state != ListingState::Sold {
    return Err(ErrorCode::ListingNotSold.into());
  }

  let seeds = auction_manager_seeds!(ctx.accounts.auction_manager);

  let referral_count = ctx.accounts.referral_recipient.count;
  let total_referral_count = ctx.accounts.listing.total_referral_count;
  let reward_percentage = ctx.accounts.auction_manager.reward_percentage / 100;
  let listing_sale = ctx.accounts.listing.bid_amount;

  // Calculate the reward amount based on the referral count and reward percentage and listing sale. Reward amount = ((listing sale * reward percentage) / total referral count ) * referral count
  let reward_amount = ((listing_sale * reward_percentage) / total_referral_count) * referral_count;

  token::transfer(
    ctx.accounts.transfer_escrow_ctx().with_signer(&[seeds]),
    reward_amount,
  )?;

  ctx.accounts.referral_recipient.claimed = true;

  Ok(())
}
