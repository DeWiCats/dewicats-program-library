use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct UnstakeArgsV0 {
    mint: Pubkey,
    stake_reciept: Pubkey,
}

#[derive(Accounts)]
pub struct UnstakeV0<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub mint: Box<Account<'info, Mint>>,
    pub stake_reciept: AccountInfo<'info>,
}

#[allow(deprecated)]
pub fn handler(ctx: Context<UnstakeV0>, args: UnstakeArgsV0) -> Result<()> {

    // Unfreeze the mint
    // ctx.mint.set_authority(ctx.authority.key, Some(&ctx.program_id))?;

    // Delete the stake reciept
    // ctx.accounts.stake_reciept.close(ctx.accounts.authority.key)?;

    Ok(())
}

