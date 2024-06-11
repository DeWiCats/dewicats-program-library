use crate::state::*;
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct InitializeBidRecieptArgsV0 {}

#[derive(Accounts)]
pub struct InitializeBidRecieptV0<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,
  pub system_program: Program<'info, System>,
  pub listing: Box<Account<'info, ListingV0>>,
  #[account(
        init,
        payer = payer,
        space = 8 + 60 + std::mem::size_of::<BidRecieptV0>(),
        seeds = ["bid_reciept".as_bytes(), listing.key().as_ref(), payer.key().as_ref()],
        bump
    )]
  pub bid_reciept: Box<Account<'info, BidRecieptV0>>,
}

pub fn handler(
  ctx: Context<InitializeBidRecieptV0>,
  _args: InitializeBidRecieptArgsV0,
) -> Result<()> {
  ctx.accounts.bid_reciept.set_inner(BidRecieptV0 {
    listing: ctx.accounts.listing.key(),
    bidder: ctx.accounts.payer.key(),
    amount: 0,
    created_at: Clock::get()?.unix_timestamp,
    state: BidRecieptState::Pending,
    referral_recipient: None,
    bump_seed: ctx.bumps["bid_reciept"],
  });

  Ok(())
}
