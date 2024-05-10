use crate::{metaplex::MetadataAccount, state::*};
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::Transfer;
use anchor_spl::{
  associated_token::AssociatedToken,
  token::{self, Token, TokenAccount},
};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct ListNftArgsV0 {
  pub starting_price: u64,
  pub duration: i64,
}

#[derive(Accounts)]
#[instruction(args: ListNftArgsV0)]
pub struct ListNftV0<'info> {
  #[account(mut)]
  pub listing_authority: Signer<'info>,
  pub nft: Box<Account<'info, Mint>>,
  pub token_mint: Box<Account<'info, Mint>>,

  #[account(mut, has_one = listing_authority)]
  pub auction_manager: Box<Account<'info, AuctionManagerV0>>,
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
        seeds = ["metadata".as_bytes(), MetadataAccount::owner().as_ref(), nft.key().as_ref()],
        seeds::program = MetadataAccount::owner(),
        bump,
        constraint = metadata.collection
            .as_ref()
            .map(|col| col.verified && col.key == auction_manager.collection)
            .unwrap_or_else(|| false)
    )]
  pub metadata: Box<Account<'info, MetadataAccount>>,

  #[account(
        init_if_needed,
        payer = listing_authority,
        associated_token::mint = nft,
        associated_token::authority = auction_manager
    )]
  pub nft_escrow: Box<Account<'info, TokenAccount>>,

  pub associated_token_program: Program<'info, AssociatedToken>,
  pub system_program: Program<'info, System>,
  pub token_program: Program<'info, Token>,
  pub rent: Sysvar<'info, Rent>,
}

impl<'info> ListNftV0<'info> {
  fn transfer_escrow_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
    let cpi_accounts = Transfer {
      from: self.nft_source.to_account_info(),
      to: self.nft_escrow.to_account_info(),
      authority: self.listing_authority.to_account_info(),
    };
    CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
  }
}

#[allow(deprecated)]
pub fn handler(ctx: Context<ListNftV0>, args: ListNftArgsV0) -> Result<()> {
  token::transfer(ctx.accounts.transfer_escrow_ctx(), 1)?;

  ctx.accounts.listing.set_inner(ListingV0 {
    nft: ctx.accounts.nft.key(),
    token_mint: ctx.accounts.token_mint.key(),
    starting_price: args.starting_price,
    duration: args.duration,
    auction_manager: ctx.accounts.auction_manager.key(),
    created_at: Clock::get()?.unix_timestamp,
    highest_bid_reciept: ctx.accounts.auction_manager.key(),
    bid_amount: 0,
    nft_escrow: ctx.accounts.nft_escrow.key(),
    total_referral_count: 0,
    state: ListingState::Active,
  });

  Ok(())
}
