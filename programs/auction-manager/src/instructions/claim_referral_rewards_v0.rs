use crate::{auction_manager_seeds, error::ErrorCode, metaplex::MetadataAccount, state::*};
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
  pub owner: Signer<'info>,
  pub auction_manager: Box<Account<'info, AuctionManagerV0>>,
  #[account(
        mut,
        has_one = token_mint,
        constraint = listing.auction_manager == *auction_manager.to_account_info().key,
        constraint = listing.token_mint == *token_mint.to_account_info().key
      )]
  pub listing: Box<Account<'info, ListingV0>>,
  pub token_mint: Box<Account<'info, Mint>>,
  #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = token_mint,
        associated_token::authority = auction_manager
    )]
  pub token_escrow: Box<Account<'info, TokenAccount>>,

  pub nft: Box<Account<'info, Mint>>,

  #[account(associated_token::mint = nft, associated_token::authority = owner)]
  pub owner_token_account: Box<Account<'info, TokenAccount>>,

  #[account(
    mut,
    seeds = [b"metadata", &mpl_token_metadata::ID.as_ref(), (*nft.to_account_info().key).as_ref()],
    seeds::program = &mpl_token_metadata::ID,
      bump,
      constraint = metadata.collection
          .as_ref()
          .map(|col| col.verified && col.key == auction_manager.collection)
          .unwrap_or_else(|| false)
  )]
  pub metadata: Box<Account<'info, MetadataAccount>>,

  #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = token_mint,
        associated_token::authority = owner
    )]
  pub token_source: Box<Account<'info, TokenAccount>>,
  #[account(mut,
      constraint = referral_recipient.claimed == false,
      constraint = referral_recipient.nft == nft.key()
    )]
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
  // Verify NFT owner is the signer
  assert_eq!(
    ctx.accounts.owner_token_account.owner,
    ctx.accounts.owner.key()
  );
  assert_eq!(
    ctx.accounts.owner_token_account.mint,
    ctx.accounts.nft.key()
  );
  assert_eq!(ctx.accounts.owner_token_account.amount, 1);

  if ctx.accounts.listing.state != ListingState::Sold {
    return Err(ErrorCode::ListingNotSold.into());
  }

  let seeds = auction_manager_seeds!(ctx.accounts.auction_manager);

  let referral_count = ctx.accounts.referral_recipient.count;
  let total_referral_count = ctx.accounts.listing.total_referral_count;
  // This cannot be a decimal
  let reward_percentage = ctx.accounts.listing.reward_percentage;
  let listing_sale = ctx.accounts.listing.bid_amount;

  // Calculate the reward amount based on the referral count and reward percentage and listing sale. Reward amount = ((listing sale * reward percentage) / total referral count ) * referral count
  let reward_amount = (listing_sale / reward_percentage / total_referral_count) * referral_count;

  token::transfer(
    ctx.accounts.transfer_escrow_ctx().with_signer(&[seeds]),
    reward_amount,
  )?;

  ctx.accounts.referral_recipient.claimed = true;

  Ok(())
}
