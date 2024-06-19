use crate::state::*;
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct UpdateListingArgsV0 {
  pub auction_proceeds_wallet: Pubkey,
  pub reward_percentage: u64,
}

#[derive(Accounts)]
#[instruction(args: UpdateListingArgsV0)]
pub struct UpdateListingV0<'info> {
  pub update_authority: Signer<'info>,
  pub system_program: Program<'info, System>,
  #[account(
        mut,
        has_one = update_authority,
        constraint = auction_manager.update_authority == update_authority.to_account_info().key()
    )]
  pub auction_manager: Box<Account<'info, AuctionManagerV0>>,

  #[account(
    mut,
    constraint = listing.auction_manager == *auction_manager.to_account_info().key,
    constraint = listing.state == ListingState::Active
  )]
  pub listing: Box<Account<'info, ListingV0>>,
}

pub fn handler(ctx: Context<UpdateListingV0>, args: UpdateListingArgsV0) -> Result<()> {
  ctx.accounts.listing.auction_proceeds_wallet = args.auction_proceeds_wallet;
  ctx.accounts.listing.reward_percentage = args.reward_percentage;

  Ok(())
}
