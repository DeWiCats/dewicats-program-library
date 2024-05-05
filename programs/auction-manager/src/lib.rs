use anchor_lang::prelude::*;
#[cfg(not(feature = "no-entrypoint"))]
declare_id!("aucwFHspXAnpzCcgCDnepFisGixxxEhW4rkBVizjXcg");

pub mod error;
pub mod instructions;
pub mod metaplex;
pub mod state;

pub use instructions::*;
pub use state::*;

#[program]
pub mod auction_manager {
    use super::*;

    pub fn initialize_manager_v0(
        ctx: Context<InitializeManagerV0>,
        args: InitializeManagerArgsV0
    ) -> Result<()> {
        initialize_manager_v0::handler(ctx, args)
    }

    pub fn list_nft_v0(ctx: Context<ListNftV0>, args: ListNftArgsV0) -> Result<()> {
        list_nft_v0::handler(ctx, args)
    }

    pub fn initialize_bid_reciept_v0(
        ctx: Context<InitializeBidRecieptV0>,
        args: InitializeBidRecieptArgsV0
    ) -> Result<()> {
        initialize_bid_reciept_v0::handler(ctx, args)
    }

    pub fn place_bid_v0(ctx: Context<PlaceBidV0>, args: PlaceBidArgsV0) -> Result<()> {
        place_bid_v0::handler(ctx, args)
    }

    pub fn cancel_bid_v0(ctx: Context<CancelBidV0>, args: CancelBidArgsV0) -> Result<()> {
        cancel_bid_v0::handler(ctx, args)
    }

    pub fn execute_sale_v0(ctx: Context<ExecuteSaleV0>, args: ExecuteSaleArgsV0) -> Result<()> {
        execute_sale_v0::handler(ctx, args)
    }

    pub fn initialize_referral_recipient_v0(
        ctx: Context<InitializeReferralRecipientV0>,
        args: InitializeReferralRecipientArgsV0
    ) -> Result<()> {
        initialize_referral_recipient_v0::handler(ctx, args)
    }

    pub fn claim_referral_rewards_v0(
        ctx: Context<ClaimReferralRewardsV0>,
        args: ClaimReferralRewardsArgsV0
    ) -> Result<()> {
        claim_referral_rewards_v0::handler(ctx, args)
    }
}
