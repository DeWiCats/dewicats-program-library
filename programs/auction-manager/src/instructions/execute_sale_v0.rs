use crate::{auction_manager_seeds, state::*};
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::Transfer;
use anchor_spl::{
  associated_token::AssociatedToken,
  token::{self, Token, TokenAccount},
};

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Default)]
pub struct ExecuteSaleArgsV0 {}

#[derive(Accounts)]
pub struct ExecuteSaleV0<'info> {
  #[account(mut)]
  pub listing_authority: Signer<'info>,
  #[account(
        mut,
        has_one = auction_proceeds_wallet,
        has_one = listing_authority
    )]
  pub auction_manager: Box<Account<'info, AuctionManagerV0>>,
  pub token_mint: Box<Account<'info, Mint>>,
  #[account(
        mut,
        has_one = token_mint,
        has_one = highest_bid_reciept
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

  pub associated_token_program: Program<'info, AssociatedToken>,
  pub system_program: Program<'info, System>,
  pub token_program: Program<'info, Token>,
  pub rent: Sysvar<'info, Rent>,
}

impl<'info> ExecuteSaleV0<'info> {
  fn transfer_escrow_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
    let cpi_accounts = Transfer {
      from: self.nft_escrow.to_account_info(),
      to: self.nft_recipient.to_account_info(),
      authority: self.auction_manager.to_account_info(),
    };
    CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
  }

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
  let seeds = auction_manager_seeds!(ctx.accounts.auction_manager);

  ctx.accounts.listing.state = ListingState::Sold;

  // transfer nft to recipient
  token::transfer(ctx.accounts.transfer_escrow_ctx().with_signer(&[seeds]), 1)?;

  // transfer auction proceeds to auction proceeds wallet
  token::transfer(
    ctx
      .accounts
      .transfer_auction_proceeds_ctx()
      .with_signer(&[seeds]),
    ctx.accounts.highest_bid_reciept.amount,
  )?;

  ctx.accounts.highest_bid_reciept.state = BidRecieptState::Executed;

  Ok(())
}
