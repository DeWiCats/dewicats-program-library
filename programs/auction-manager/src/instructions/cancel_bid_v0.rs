use crate::{auction_manager_seeds, state::*};
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::Transfer;
use anchor_spl::{
  associated_token::AssociatedToken,
  token::{self, Token, TokenAccount},
};

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Default)]
pub struct CancelBidArgsV0 {
  pub bid_reciept: Pubkey,
}

#[derive(Accounts)]
pub struct CancelBidV0<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,
  pub listing: Box<Account<'info, ListingV0>>,
  pub auction_manager: Box<Account<'info, AuctionManagerV0>>,
  #[account(
        mut,
        has_one = listing
      )]
  pub bid_reciept: Box<Account<'info, BidRecieptV0>>,
  pub token_mint: Box<Account<'info, Mint>>,
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

  pub associated_token_program: Program<'info, AssociatedToken>,
  pub system_program: Program<'info, System>,
  pub token_program: Program<'info, Token>,
  pub rent: Sysvar<'info, Rent>,
}

impl<'info> CancelBidV0<'info> {
  fn transfer_escrow_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
    let cpi_accounts = Transfer {
      from: self.token_escrow.to_account_info(),
      to: self.token_source.to_account_info(),
      authority: self.auction_manager.to_account_info(),
    };
    CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
  }
}

pub fn handler(ctx: Context<CancelBidV0>, _args: CancelBidArgsV0) -> Result<()> {
  let seeds = auction_manager_seeds!(ctx.accounts.auction_manager);
  token::transfer(
    ctx.accounts.transfer_escrow_ctx().with_signer(&[seeds]),
    ctx.accounts.bid_reciept.amount,
  )?;
  ctx.accounts.bid_reciept.state = BidRecieptState::Cancelled;

  Ok(())
}
