use crate::{error::ErrorCode, state::*};
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::Transfer;
use anchor_spl::{
  associated_token::AssociatedToken,
  token::{self, Token, TokenAccount},
};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct PlaceBidArgsV0 {
  amount: u64,
}

#[derive(Accounts)]
pub struct PlaceBidV0<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,
  pub auction_manager: Box<Account<'info, AuctionManagerV0>>,
  #[account(
        mut,
        has_one = token_mint,
        constraint = listing.auction_manager == *auction_manager.to_account_info().key,
        constraint = listing.token_mint == *token_mint.to_account_info().key,
        constraint = listing.state == ListingState::Active
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
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = payer,
      )]
  pub token_source: Box<Account<'info, TokenAccount>>,
  #[account(mut)]
  pub referral_recipient: Option<Box<Account<'info, ReferralRecipientV0>>>,
  pub associated_token_program: Program<'info, AssociatedToken>,
  pub system_program: Program<'info, System>,
  pub token_program: Program<'info, Token>,
  pub rent: Sysvar<'info, Rent>,
}

impl<'info> PlaceBidV0<'info> {
  fn transfer_escrow_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
    let cpi_accounts = Transfer {
      from: self.token_source.to_account_info(),
      to: self.token_escrow.to_account_info(),
      authority: self.payer.to_account_info(),
    };
    CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
  }
}

pub fn handler(ctx: Context<PlaceBidV0>, args: PlaceBidArgsV0) -> Result<()> {
  // Check that bid is less than starting_price
  if ctx.accounts.listing.starting_price > args.amount {
    return Err(ErrorCode::BidAmountTooLow.into());
  }

  // check if bid amount is greater than current bid amount
  if ctx.accounts.listing.bid_amount >= args.amount {
    return Err(ErrorCode::BidAmountTooLow.into());
  }

  // check that listing end_at i64 and created_at timestamp is greater than current time
  if ctx.accounts.listing.end_at < Clock::get()?.unix_timestamp {
    return Err(ErrorCode::ListingExpired.into());
  }

  // only transfer escrow the remaining amount if a bid reciept has already been created
  if ctx.accounts.bid_reciept.state == BidRecieptState::Active {
    token::transfer(
      ctx.accounts.transfer_escrow_ctx(),
      args.amount - ctx.accounts.bid_reciept.amount,
    )?;
  } else {
    token::transfer(ctx.accounts.transfer_escrow_ctx(), args.amount)?;
  }

  ctx.accounts.bid_reciept.amount = args.amount;
  ctx.accounts.listing.bid_amount = args.amount;
  ctx.accounts.listing.highest_bid_reciept = ctx.accounts.bid_reciept.key();
  ctx.accounts.bid_reciept.state = BidRecieptState::Active;
  ctx.accounts.bid_reciept.created_at = Clock::get()?.unix_timestamp;

  // If bid is within 5 minutes of listing end, extend listing end_at to a maximum of 5 minutes
  let end_timestamp = ctx.accounts.listing.end_at;
  let current_timestamp = Clock::get()?.unix_timestamp;
  let time_extension = ctx.accounts.listing.time_extension as i64;
  if end_timestamp - current_timestamp < time_extension && end_timestamp - current_timestamp > 0 {
    // Get the difference between the current time and the time extension
    let time_extension_diff = time_extension - (end_timestamp - current_timestamp);
    // If the difference is greater than 5 minutes, set the time extension to 5 minutes
    ctx.accounts.listing.end_at += time_extension_diff;
  }

  if let Some(ref mut referral_recipient) = ctx.accounts.referral_recipient {
    // Check that the referral_recipient listing is the same as the current listing
    if referral_recipient.listing != ctx.accounts.listing.key() {
      return Err(ErrorCode::ReferralRecipientListingMismatch.into());
    }
    // if ctx.accounts.bid_reciept.referral_recipient exists already then dont increase total_referral_count
    if ctx.accounts.bid_reciept.referral_recipient.is_none() {
      referral_recipient.count += 1;
      ctx.accounts.listing.total_referral_count += 1;
    }

    ctx.accounts.bid_reciept.referral_recipient = Some(referral_recipient.key());
  }

  Ok(())
}
