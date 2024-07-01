use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
  #[msg("The auction has already ended")]
  ListingNotActive,

  #[msg("The auction has already ended")]
  ListingExpired,

  #[msg("The bid amount is too low")]
  BidAmountTooLow,

  #[msg("Referral recipient not found")]
  ReferralRecipientNotFound,

  #[msg("Listing is not sold")]
  ListingNotSold,

  #[msg("Listing still active")]
  ListingStillActive,

  #[msg("Referral recipient listing does not match bid reciept listing")]
  ReferralRecipientListingMismatch,

  #[msg("Bid already cancelled")]
  BidAlreadyCancelled,

  #[msg("Referral recipient is not the same as the previous referral recipient")]
  ReferralRecipientDifferentThanPrevious,

  #[msg("Referral recipient in bid reciept but new bid has no referral recipient")]
  ReferralRecipientInBidRecieptButNewBidHasNoReferralRecipient,
}
