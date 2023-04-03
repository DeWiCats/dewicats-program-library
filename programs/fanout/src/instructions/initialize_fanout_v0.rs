use anchor_lang::prelude::*;
use anchor_spl::{
  associated_token::AssociatedToken,
  metadata::{create_master_edition_v3, CreateMasterEditionV3, CreateMetadataAccountsV3, Metadata},
  token::{self, Mint, MintTo, Token, TokenAccount},
};
use mpl_token_metadata::state::{CollectionDetails, DataV2};
use shared_utils::create_metadata_accounts_v3;

use crate::FanoutV0;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct InitializeFanoutArgsV0 {
  pub name: String,
}

#[derive(Accounts)]
#[instruction(args: InitializeFanoutArgsV0)]
pub struct InitializeFanoutV0<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,
  /// CHECK: Deposit the collection
  pub authority: AccountInfo<'info>,

  #[account(
    init,
    payer = payer,
    space = 60 + 8 + std::mem::size_of::<FanoutV0>() + args.name.len(),
    seeds = ["fanout".as_bytes(), args.name.as_bytes()],
    bump
  )]
  pub fanout: Box<Account<'info, FanoutV0>>,
  #[account(
    init_if_needed,
    payer = payer,
    associated_token::mint = fanout_mint,
    associated_token::authority = fanout,
  )]
  pub token_account: Box<Account<'info, TokenAccount>>,
  pub fanout_mint: Box<Account<'info, Mint>>,
  pub collection: Box<Account<'info, Mint>>,
  pub membership_mint: Box<Account<'info, Mint>>,
  
  pub token_program: Program<'info, Token>,
  pub associated_token_program: Program<'info, AssociatedToken>,
  pub system_program: Program<'info, System>,
  // DO I need this?
  // pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<InitializeFanoutV0>, args: InitializeFanoutArgsV0) -> Result<()> {
  ctx.accounts.fanout.set_inner(FanoutV0 {
    authority: ctx.accounts.authority.key(),
    token_account: ctx.accounts.token_account.key(),
    membership_mint: ctx.accounts.membership_mint.key(),
    fanout_mint: ctx.accounts.fanout_mint.key(),
    membership_collection: ctx.accounts.collection.key(),
    name: args.name,
    total_shares: ctx.accounts.membership_mint.supply,
    last_snapshot_amount: ctx.accounts.token_account.amount,
    total_inflow: ctx.accounts.token_account.amount,
    bump_seed: ctx.bumps["fanout"],
  });

  Ok(())
}
