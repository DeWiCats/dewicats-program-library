use crate::state::*;
use anchor_lang::prelude::*;

declare_id!("hemjuPXBpNvggtaUnN1MwT3wrdhttKEfosTcc2P9Pg8");

pub mod error;
pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;

#[program]
pub mod stake_manager {
    use super::*;

    pub fn stake_v0(ctx: Context<StakeV0>, args: StakeArgsV0) -> Result<()> {
        stake_v0::handler(ctx, args)
    }

    // pub fn unstake_v0(ctx: Context<UnstakeV0>, args: UnstakeArgsV0) -> Result<()> {
    //     unstake_v0::handler(ctx, args)
    // }
}