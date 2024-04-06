use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct StakeArgsV0 {
    mint: Pubkey,
    stake_reciept: Pubkey,
    collection: Pubkey,
}

#[derive(Accounts)]
pub struct StakeV0<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub mint: Box<Account<'info, Mint>>,
    #[account(
        init,
        payer = authority,
        space = 8 + 60 + std::mem::size_of::<StakeRecieptV0>(),
        seeds = ["stake_reciept".as_bytes(), mint.key().as_ref(), authority.key().as_ref()],
        bump,
    )]
    pub stake_reciept: AccountInfo<'info>,
    pub collection: Box<Account<'info, Mint>>,
}

#[allow(deprecated)]
pub fn handler(ctx: Context<StakeV0>, args: StakeArgsV0) -> Result<()> {        

    // Freeze the mint
    // ctx.mint.set_authority(ctx.authority.key, None)?;

    // ctx.stake.set_inner(StakeRecieptV0 {
    //     mint: *ctx.mint.key,
    //     created_at: Clock::get()?.unix_timestamp,
    // });

    Ok(())
}
