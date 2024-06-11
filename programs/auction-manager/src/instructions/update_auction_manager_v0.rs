use crate::state::*;
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct UpdateAuctionManagerArgsV0 {
  pub update_authority: Pubkey,
  pub listing_authority: Pubkey,
  pub reward_percentage: u64,
}

#[derive(Accounts)]
#[instruction(args: UpdateAuctionManagerArgsV0)]
pub struct UpdateAuctionManagerV0<'info> {
  pub update_authority: Signer<'info>,
  pub system_program: Program<'info, System>,
  #[account(
        mut,
        has_one = update_authority,
        constraint = auction_manager.update_authority == update_authority.to_account_info().key()
    )]
  pub auction_manager: Box<Account<'info, AuctionManagerV0>>,
}

pub fn handler(
  ctx: Context<UpdateAuctionManagerV0>,
  args: UpdateAuctionManagerArgsV0,
) -> Result<()> {
  ctx.accounts.auction_manager.update_authority = args.update_authority;
  ctx.accounts.auction_manager.listing_authority = args.listing_authority;
  ctx.accounts.auction_manager.reward_percentage = args.reward_percentage;

  Ok(())
}
