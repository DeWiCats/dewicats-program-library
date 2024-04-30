import { Program } from "@coral-xyz/anchor";
import { AuctionManager } from "../../target/types/auction_manager";
import { execSync } from "child_process";

export async function ensureAuctionIdl(
  auctionProgram: Program<AuctionManager>
) {
  try {
    execSync(
      `anchor idl init --filepath ${__dirname}/../../target/idl/auction_manager.json ${auctionProgram.programId}`,
      { stdio: "inherit", shell: "/bin/bash" }
    );
  } catch {
    execSync(
      `anchor idl upgrade --filepath ${__dirname}/../../target/idl/auction_manager.json ${auctionProgram.programId}`,
      { stdio: "inherit", shell: "/bin/bash" }
    );
  }
}
