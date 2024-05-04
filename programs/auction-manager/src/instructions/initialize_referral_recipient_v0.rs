use crate::{metaplex::MetadataAccount, state::*};
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct InitializeReferralRecipientArgsV0 {
  referral_code: String,
}

#[derive(Accounts)]
#[instruction(args: InitializeReferralRecipientArgsV0)]
pub struct InitializeReferralRecipientV0<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,
  pub system_program: Program<'info, System>,
  pub nft: Box<Account<'info, Mint>>,
  pub auction_manager: Box<Account<'info, AuctionManagerV0>>,
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
  pub listing: Box<Account<'info, ListingV0>>,
  #[account(
        init,
        payer = payer,
        space = 8 + 60 + std::mem::size_of::<InitializeReferralRecipientV0>(),
        seeds = ["referral_recipient".as_bytes(), nft.key().as_ref(), listing.key().as_ref()],
        bump
    )]
  pub referral_recipient: Box<Account<'info, ReferralRecipientV0>>,
}

pub fn handler(
  ctx: Context<InitializeReferralRecipientV0>,
  args: InitializeReferralRecipientArgsV0,
) -> Result<()> {
  ctx
    .accounts
    .referral_recipient
    .set_inner(ReferralRecipientV0 {
      referral_code: args.referral_code,
      nft: ctx.accounts.nft.key(),
      count: 0,
      claimed: false,
    });

  Ok(())
}