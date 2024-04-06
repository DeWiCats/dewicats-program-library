use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct StakeRecieptV0 {
    pub mint: Pubkey,
    pub created_at: i64,
}