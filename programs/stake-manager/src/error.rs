use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
  #[msg("NFT not found in DeWiCats collection")]
  InvalidDataIncrease,
}
