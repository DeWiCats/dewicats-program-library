import {
  ataResolver,
  combineResolvers,
  heliumCommonResolver,
  resolveIndividual,
} from "@helium/anchor-resolvers";
import { PublicKey } from "@solana/web3.js";

const METADATA_PROGRAM_ID = new PublicKey(
  "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);

export const auctionManagerResolvers = combineResolvers(
  heliumCommonResolver,
  resolveIndividual(async ({ path, provider, accounts, programId }) => {
    if (path[path.length - 1] == "metadata" && accounts.nft) {
      return PublicKey.findProgramAddressSync(
        [
          Buffer.from("metadata", "utf-8"),
          METADATA_PROGRAM_ID.toBuffer(),
          (accounts.nft as PublicKey).toBuffer(),
        ],
        METADATA_PROGRAM_ID
      )[0];
    }
  }),
  ataResolver({
    instruction: "listNftV0",
    account: "tokenEscrow",
    mint: "tokenMint",
    owner: "auctionManager",
  }),
  ataResolver({
    instruction: "listNftV0",
    account: "nftSource",
    mint: "nft",
    owner: "listingAuthority",
  }),
  ataResolver({
    instruction: "listNftV0",
    account: "nftEscrow",
    mint: "nft",
    owner: "auctionManager",
  }),

  ataResolver({
    instruction: "placeBidV0",
    account: "tokenEscrow",
    mint: "tokenMint",
    owner: "auctionManager",
  }),
  ataResolver({
    instruction: "placeBidV0",
    account: "tokenSource",
    mint: "tokenMint",
    owner: "payer",
  }),

  ataResolver({
    instruction: "cancelBidV0",
    account: "tokenEscrow",
    mint: "tokenMint",
    owner: "auctionManager",
  }),
  ataResolver({
    instruction: "cancelBidV0",
    account: "tokenSource",
    mint: "tokenMint",
    owner: "payer",
  }),

  ataResolver({
    instruction: "executeSaleV0",
    account: "nftRecipient",
    mint: "nft",
    owner: "bidder",
  }),
  ataResolver({
    instruction: "executeSaleV0",
    account: "nftEscrow",
    mint: "nft",
    owner: "auctionManager",
  }),
  ataResolver({
    instruction: "executeSaleV0",
    account: "auctionProceedsTokenAccount",
    mint: "tokenMint",
    owner: "auctionProceedsWallet",
  }),
  ataResolver({
    instruction: "executeSaleV0",
    account: "tokenEscrow",
    mint: "tokenMint",
    owner: "auctionManager",
  }),
  ataResolver({
    instruction: "claimReferralRewardsV0",
    account: "tokenEscrow",
    mint: "tokenMint",
    owner: "auctionManager",
  }),
  ataResolver({
    instruction: "claimReferralRewardsV0",
    account: "tokenSource",
    mint: "tokenMint",
    owner: "payer",
  })
);
