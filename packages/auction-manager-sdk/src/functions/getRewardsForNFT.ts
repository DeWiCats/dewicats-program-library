import { AnchorProvider } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { init } from "../init";

export const getRewardsForNFT = async (
  referralRecipient: PublicKey,
  listing: PublicKey,
  provider: AnchorProvider
) => {
  // Get rewards for NFT
  const auctionManager = await init(provider);

  const listingAcc = await auctionManager.account.listingV0.fetch(listing);
  const referralRecipientAcc =
    await auctionManager.account.referralRecipientV0.fetch(referralRecipient);

  const referralCount = referralRecipientAcc.count.toNumber();
  const listingRewardPercentage = listingAcc.rewardPercentage.toNumber();
  const listingTotalReferralCount = listingAcc.totalReferralCount.toNumber();
  const listingSalePrice = listingAcc.bidAmount.toNumber();

  const pendingRewards =
    listingSalePrice *
    (listingRewardPercentage / 100) *
    (referralCount / listingTotalReferralCount);

  return pendingRewards;
};
