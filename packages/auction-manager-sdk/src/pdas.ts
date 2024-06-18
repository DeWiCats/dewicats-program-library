import { PublicKey } from "@solana/web3.js";
import { PROGRAM_ID } from "./constants";

/**
 *
 * @param listing
 * @param bidder
 * @param programId
 * @returns
 */
export const bidderRecieptKey = (
  listing: PublicKey,
  bidder: PublicKey,
  programId: PublicKey = PROGRAM_ID
) =>
  PublicKey.findProgramAddressSync(
    [
      Buffer.from("bid_reciept", "utf-8"),
      listing.toBuffer(),
      bidder.toBuffer(),
    ],
    programId
  );

/**
 *
 * @param listing
 * @param nft
 * @param programId
 * @returns
 */
export const referralRecipientKey = (
  listing: PublicKey,
  nftName: string,
  programId: PublicKey = PROGRAM_ID
) => {
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from("referral_recipient", "utf-8"),
      Buffer.from(nftName, "utf-8"),
      listing.toBuffer(),
    ],
    programId
  );
};

/**
 *
 * @param collection
 * @param name
 * @param programId
 * @returns
 */
export const auctionManagerKey = (
  collection: PublicKey,
  name: string,
  programId: PublicKey = PROGRAM_ID
) =>
  PublicKey.findProgramAddressSync(
    [
      Buffer.from("auction_manager", "utf-8"),
      collection.toBuffer(),
      Buffer.from(name, "utf-8"),
    ],
    programId
  );
