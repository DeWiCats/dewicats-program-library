use crate::{metaplex::MetadataAccount, state::*, transfer_pnft::transfer_pnft_with_signer};
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::{
  associated_token::AssociatedToken,
  token::{Token, TokenAccount},
};
use mpl_token_metadata::instruction::TransferArgs;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct ListNftArgsV0 {
  pub starting_price: u64,
  pub end_at: i64,
  pub auction_proceeds_wallet: Pubkey,
  pub reward_percentage: u64,
  pub time_extension: u64,
}

#[derive(Accounts)]
#[instruction(args: ListNftArgsV0)]
pub struct ListNftV0<'info> {
  #[account(mut)]
  pub listing_authority: Signer<'info>,
  pub nft: Box<Account<'info, Mint>>,
  pub token_mint: Box<Account<'info, Mint>>,

  #[account(
      mut,
      has_one = listing_authority,
      constraint = auction_manager.listing_authority == listing_authority.to_account_info().key())]
  pub auction_manager: Box<Account<'info, AuctionManagerV0>>,
  #[account(
        init,
        payer = listing_authority,
        space = 8 + 60 + std::mem::size_of::<BidRecieptV0>(),
        seeds = [
            "bid_reciept".as_bytes(),
            listing.key().as_ref(),
            listing_authority.key().as_ref(),
        ],
        bump
    )]
  pub initial_bid_reciept: Box<Account<'info, BidRecieptV0>>,
  #[account(
        init,
        payer = listing_authority,
        space = 8 + 60 + std::mem::size_of::<ListingV0>(),
        seeds = ["listing".as_bytes(), nft.key().as_ref(), auction_manager.key().as_ref()],
        bump
    )]
  pub listing: Box<Account<'info, ListingV0>>,

  #[account(
        init_if_needed,
        payer = listing_authority,
        associated_token::mint = token_mint,
        associated_token::authority = auction_manager
    )]
  pub token_escrow: Box<Account<'info, TokenAccount>>,

  #[account(
        mut,
        associated_token::mint = nft,
        associated_token::authority = listing_authority,
      )]
  pub nft_source: Box<Account<'info, TokenAccount>>,

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
      mut,
      seeds = [b"metadata", &mpl_token_metadata::ID.as_ref(), (*nft.to_account_info().key).as_ref(), b"edition"],
      seeds::program = &mpl_token_metadata::ID,
      bump,
  )]
  /// CHECK: This is not dangerous because we don't read or write from this account
  pub nft_edition: AccountInfo<'info>,

  #[account(
      mut,
      // constraint = owner_token_record.owner == &mpl_token_metadata::ID
  )]
  /// CHECK: This is not dangerous because we don't read or write from this account
  pub owner_token_record: AccountInfo<'info>,

  #[account(mut)]
  /// CHECK: This is not dangerous because we don't read or write from this account
  pub destination_token_record: AccountInfo<'info>,

  #[account(mut)]
  /// CHECK: This is not dangerous because we don't read or write from this account
  pub authorization_rules: AccountInfo<'info>,

  /// CHECK: This is not dangerous because we don't read or write from this account
  pub mpl_token_auth_rules_program: AccountInfo<'info>,

  #[account(
        init_if_needed,
        payer = listing_authority,
        associated_token::mint = nft,
        associated_token::authority = auction_manager
    )]
  pub nft_escrow: Box<Account<'info, TokenAccount>>,

  /// CHECK: Should be checked by the metaplex instruction
  pub instructions: UncheckedAccount<'info>,
  /// CHECK: Verified by constraint
  #[account(address = mpl_token_metadata::ID)]
  pub token_metadata_program: AccountInfo<'info>,
  pub associated_token_program: Program<'info, AssociatedToken>,
  pub system_program: Program<'info, System>,
  pub token_program: Program<'info, Token>,
  pub rent: Sysvar<'info, Rent>,
}

#[allow(deprecated)]
pub fn handler(ctx: Context<ListNftV0>, args: ListNftArgsV0) -> Result<()> {
  let transfer_args = TransferArgs::V1 {
    amount: 1,
    authorization_data: None,
  };

  transfer_pnft_with_signer(
    ctx.accounts.nft_source.to_account_info(),
    ctx.accounts.listing_authority.to_account_info(),
    ctx.accounts.nft_escrow.to_account_info(),
    ctx.accounts.auction_manager.to_account_info().clone(),
    ctx.accounts.listing_authority.to_account_info(),
    ctx.accounts.listing_authority.to_account_info(),
    ctx.accounts.nft.to_account_info(),
    ctx.accounts.metadata.to_account_info(),
    ctx.accounts.nft_edition.to_account_info(),
    ctx.accounts.owner_token_record.to_account_info(),
    ctx.accounts.destination_token_record.to_account_info(),
    ctx.accounts.authorization_rules.to_account_info(),
    ctx.accounts.mpl_token_auth_rules_program.to_account_info(),
    ctx.accounts.token_program.to_account_info(),
    ctx.accounts.token_metadata_program.to_account_info(),
    ctx.accounts.associated_token_program.to_account_info(),
    ctx.accounts.system_program.to_account_info(),
    ctx.accounts.instructions.to_account_info(),
    transfer_args,
    None,
  )?;

  ctx.accounts.initial_bid_reciept.set_inner(BidRecieptV0 {
    listing: ctx.accounts.listing.key(),
    bidder: ctx.accounts.listing_authority.key(),
    amount: args.starting_price,
    created_at: Clock::get()?.unix_timestamp,
    state: BidRecieptState::Pending,
    referral_recipient: None,
    bump_seed: ctx.bumps["initial_bid_reciept"],
  });

  ctx.accounts.listing.set_inner(ListingV0 {
    nft: ctx.accounts.nft.key(),
    token_mint: ctx.accounts.token_mint.key(),
    starting_price: args.starting_price,
    end_at: args.end_at,
    auction_manager: ctx.accounts.auction_manager.key(),
    created_at: Clock::get()?.unix_timestamp,
    highest_bid_reciept: ctx.accounts.listing_authority.key(),
    bid_amount: args.starting_price,
    nft_escrow: ctx.accounts.nft_escrow.key(),
    total_referral_count: 0,
    state: ListingState::Active,
    auction_proceeds_wallet: args.auction_proceeds_wallet,
    reward_percentage: args.reward_percentage,
    time_extension: args.time_extension,
    bump_seed: ctx.bumps["listing"],
  });

  Ok(())
}
