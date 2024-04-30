import { AuctionManager } from "@dewicats/idls/lib/types/auction_manager";
import { AnchorProvider, Idl, Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { PROGRAM_ID } from "./constants";
import { auctionManagerResolvers } from "./resolvers";

export const init = async (
  provider: AnchorProvider,
  programId: PublicKey = PROGRAM_ID,
  idl?: Idl | null
): Promise<Program<AuctionManager>> => {
  if (!idl) {
    idl = await Program.fetchIdl(programId, provider);
  }

  const auctionManager = new Program<AuctionManager>(
    idl as AuctionManager,
    programId,
    provider,
    undefined,
    () => auctionManagerResolvers
  ) as Program<AuctionManager>;

  return auctionManager;
};
