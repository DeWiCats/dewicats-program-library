use crate::{
  auction_manager_seeds, error::ErrorCode, metaplex::MetadataAccount, state::*,
  transfer_pnft::transfer_pnft_with_signer,
};

use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::Transfer;
use anchor_spl::{
  associated_token::AssociatedToken,
  token::{self, Token, TokenAccount},
};
use mpl_token_metadata::instruction::TransferArgs;

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Default)]
pub struct ExecuteSaleArgsV0 {}

#[derive(Accounts)]
pub struct ExecuteSaleV0<'info> {
  #[account(mut)]
  pub listing_authority: Signer<'info>,
  #[account(
        mut,
        has_one = listing_authority,
        constraint = auction_manager.listing_authority == listing_authority.to_account_info().key()
    )]
  pub auction_manager: Box<Account<'info, AuctionManagerV0>>,
  pub token_mint: Box<Account<'info, Mint>>,
  #[account(
        mut,
        has_one = token_mint,
        has_one = highest_bid_reciept,
        has_one = auction_proceeds_wallet,
        constraint = listing.auction_proceeds_wallet == auction_proceeds_wallet.key()
      )]
  pub listing: Box<Account<'info, ListingV0>>,
  #[account(
        mut,
        has_one = bidder
      )]
  pub highest_bid_reciept: Box<Account<'info, BidRecieptV0>>,
  pub nft: Box<Account<'info, Mint>>,
  /// CHECK: Verified on bid reciept
  pub bidder: UncheckedAccount<'info>,

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
        associated_token::authority = bidder
    )]
  pub nft_recipient: Box<Account<'info, TokenAccount>>,
  pub collection: Box<Account<'info, Mint>>,

  #[account(
        init_if_needed,
        payer = listing_authority,
        associated_token::mint = nft,
        associated_token::authority = auction_manager
    )]
  pub nft_escrow: Box<Account<'info, TokenAccount>>,

  #[account(
        init_if_needed,
        payer = listing_authority,
        associated_token::mint = token_mint,
        associated_token::authority = auction_proceeds_wallet
    )]
  pub auction_proceeds_token_account: Box<Account<'info, TokenAccount>>,

  #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = auction_manager,
      )]
  pub token_escrow: Box<Account<'info, TokenAccount>>,

  /// CHECK: Verified on auction manager
  pub auction_proceeds_wallet: AccountInfo<'info>,

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

impl<'info> ExecuteSaleV0<'info> {
  fn transfer_auction_proceeds_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
    let cpi_accounts = Transfer {
      from: self.token_escrow.to_account_info(),
      to: self.auction_proceeds_token_account.to_account_info(),
      authority: self.auction_manager.to_account_info(),
    };
    CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
  }
}

#[allow(deprecated)]
pub fn handler(ctx: Context<ExecuteSaleV0>, _args: ExecuteSaleArgsV0) -> Result<()> {
  // Verify that the listing has ended
  let end_at = ctx.accounts.listing.end_at;
  let created_at = ctx.accounts.listing.created_at;
  msg!(
    "reward percentage: {}",
    ctx.accounts.listing.reward_percentage
  );
  msg!("end_at: {}", end_at);
  msg!("created_at: {}", created_at);
  msg!("unix_timestamp: {}", Clock::get()?.unix_timestamp);

  if end_at >= Clock::get()?.unix_timestamp {
    return Err(ErrorCode::ListingStillActive.into());
  }

  let seeds = auction_manager_seeds!(ctx.accounts.auction_manager);

  ctx.accounts.listing.state = ListingState::Sold;

  let transfer_args = TransferArgs::V1 {
    amount: 1,
    authorization_data: None,
  };

  transfer_pnft_with_signer(
    ctx.accounts.nft_escrow.to_account_info(),
    ctx.accounts.auction_manager.to_account_info().clone(),
    ctx.accounts.nft_recipient.to_account_info(),
    ctx.accounts.bidder.to_account_info(),
    ctx.accounts.listing_authority.to_account_info(),
    ctx.accounts.auction_manager.to_account_info().clone(),
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
    Some(&[seeds]),
  )?;

  let reward_percentage = ctx.accounts.listing.reward_percentage;

  let reward_amount = ctx.accounts.highest_bid_reciept.amount / reward_percentage;

  let proceeds_amount = ctx.accounts.highest_bid_reciept.amount - reward_amount;

  // transfer auction proceeds to auction proceeds wallet
  token::transfer(
    ctx
      .accounts
      .transfer_auction_proceeds_ctx()
      .with_signer(&[seeds]),
    proceeds_amount,
  )?;

  ctx.accounts.highest_bid_reciept.state = BidRecieptState::Executed;

  Ok(())
}
