import { PublicKey } from "@solana/web3.js";
import { PROGRAM_ID } from "./constants";

export const bidderRecieptKey = (
  listing: PublicKey,
  programId: PublicKey = PROGRAM_ID
) =>
  PublicKey.findProgramAddressSync(
    [Buffer.from("bid_reciept", "utf-8"), listing.toBuffer()],
    programId
  );
