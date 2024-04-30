use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct AuctionManagerV0 {
    pub collection: Pubkey,
    pub name: String,
    pub update_authority: Pubkey,
    pub listing_authority: Pubkey,
    pub auction_proceeds_wallet: Pubkey,
    pub reward_percentage: u64,
    pub bump_seed: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, PartialEq)]
pub enum BidRecieptState {
    #[default]
    Pending,
    Active,
    Cancelled,
    Executed,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, PartialEq)]
pub enum ListingState {
    #[default]
    Active,
    Cancelled,
    Sold,
}

#[account]
#[derive(Default)]
pub struct BidRecieptV0 {
    pub listing: Pubkey,
    pub bidder: Pubkey,
    pub amount: u64,
    pub created_at: i64,
    pub state: BidRecieptState,
    pub referral_recipient: Option<Pubkey>,
}

#[account]
#[derive(Default)]
pub struct ListingV0 {
    pub auction_manager: Pubkey,
    pub nft: Pubkey,
    pub token_mint: Pubkey,
    pub starting_price: u64,
    pub duration: i64,
    pub created_at: i64,
    pub highest_bid_reciept: Pubkey,
    pub bid_amount: u64,
    pub nft_escrow: Pubkey,
    pub total_referral_count: u64,
    pub state: ListingState,
}

#[account]
#[derive(Default)]
pub struct ReferralRecipientV0 {
    pub referral_code: String,
    pub nft: Pubkey,
    pub count: u64,
    pub claimed: bool,
}

#[macro_export]
macro_rules! auction_manager_seeds {
    ($auction_manager:expr) => {
        &[
            b"auction_manager".as_ref(),
            $auction_manager.collection.as_ref(),
            $auction_manager.name.as_bytes(),
            &[$auction_manager.bump_seed],
        ]
    };
}
