use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("The auction has already ended")]
    ListingNotActive,

    #[msg("The auction has already ended")]
    ListingExpired,

    #[msg("The bid amount is too low")]
    BidAmountTooLow,

    #[msg("Referral code does not match")]
    ReferralCodeMismatch,

    #[msg("Referral recipient not found")]
    ReferralRecipientNotFound,

    #[msg("Listing is not sold")]
    ListingNotSold,
}
