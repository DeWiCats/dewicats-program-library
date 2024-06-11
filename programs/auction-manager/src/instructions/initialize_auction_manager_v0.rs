use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct InitializeAuctionManagerArgsV0 {
  pub name: String,
  pub collection: Pubkey,
  pub update_authority: Pubkey,
  pub listing_authority: Pubkey,
  pub reward_percentage: u64,
}

#[derive(Accounts)]
#[instruction(args: InitializeAuctionManagerArgsV0)]
pub struct InitializeAuctionManagerV0<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,
  pub system_program: Program<'info, System>,
  pub collection: Box<Account<'info, Mint>>,
  #[account(
        init,
        payer = payer,
        space = 8 + 60 + std::mem::size_of::<AuctionManagerV0>(),
        seeds = ["auction_manager".as_bytes(), collection.key().as_ref(), args.name.as_bytes()],
        bump
    )]
  pub auction_manager: Box<Account<'info, AuctionManagerV0>>,
}

pub fn handler(
  ctx: Context<InitializeAuctionManagerV0>,
  args: InitializeAuctionManagerArgsV0,
) -> Result<()> {
  ctx.accounts.auction_manager.set_inner(AuctionManagerV0 {
    collection: args.collection,
    name: args.name,
    update_authority: args.update_authority,
    listing_authority: args.listing_authority,
    reward_percentage: args.reward_percentage,
    bump_seed: ctx.bumps["auction_manager"],
  });

  Ok(())
}
