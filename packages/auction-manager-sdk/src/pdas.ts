import { PublicKey } from "@solana/web3.js";
import { PROGRAM_ID } from "./constants";

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

export const referralRecipientKey = (
  listing: PublicKey,
  nft: PublicKey,
  programId: PublicKey = PROGRAM_ID
) => {
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from("referral_recipient", "utf-8"),
      nft.toBuffer(),
      listing.toBuffer(),
    ],
    programId
  );
};
