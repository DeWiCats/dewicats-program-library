import {
  ataResolver,
  combineResolvers,
  heliumCommonResolver,
  resolveIndividual,
} from "@helium/anchor-resolvers";
import { PublicKey } from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";

export const METADATA_PROGRAM_ID = new PublicKey(
  "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);

export const MPL_DEFAULT_RULE_SET = new PublicKey(
  "eBJLFYPxJmMGKuFwpDWkzxZeUrad92kZRC5BJLpzyT9"
);

const getMetadata = async (mint: PublicKey): Promise<PublicKey> => {
  return (
    await PublicKey.findProgramAddress(
      [
        Buffer.from("metadata"),
        METADATA_PROGRAM_ID.toBuffer(),
        mint.toBuffer(),
      ],
      METADATA_PROGRAM_ID
    )
  )[0];
};

const getMasterEdition = async (
  mint: anchor.web3.PublicKey
): Promise<anchor.web3.PublicKey> => {
  return (
    await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("metadata"),
        METADATA_PROGRAM_ID.toBuffer(),
        mint.toBuffer(),
        Buffer.from("edition"),
      ],
      METADATA_PROGRAM_ID
    )
  )[0];
};

const findTokenRecordPda = async (
  mint: PublicKey,
  token: PublicKey
): Promise<anchor.web3.PublicKey> => {
  return (
    await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("metadata"),
        METADATA_PROGRAM_ID.toBuffer(),
        mint.toBuffer(),
        Buffer.from("token_record"),
        token.toBuffer(),
      ],
      METADATA_PROGRAM_ID
    )
  )[0];
};

export const auctionManagerResolvers = combineResolvers(
  heliumCommonResolver,

  // token_program
  resolveIndividual(async ({ path, provider, accounts, programId }) => {
    if (path[path.length - 1] == "tokenProgram" && accounts.nft) {
      return PublicKey.findProgramAddressSync(
        [
          Buffer.from("metadata"),
          METADATA_PROGRAM_ID.toBuffer(),
          (accounts.nft as PublicKey).toBuffer(),
        ],
        METADATA_PROGRAM_ID
      )[0];
    }
  }),

  resolveIndividual(async ({ path, provider, accounts, programId }) => {
    if (path[path.length - 1] == "metadata" && accounts.nft) {
      return PublicKey.findProgramAddressSync(
        [
          Buffer.from("metadata"),
          METADATA_PROGRAM_ID.toBuffer(),
          (accounts.nft as PublicKey).toBuffer(),
        ],
        METADATA_PROGRAM_ID
      )[0];
    }
  }),
  resolveIndividual(async ({ path, accounts, programId }) => {
    if (path[path.length - 1] == "nftEdition" && accounts.nft) {
      const nftEdition = await getMasterEdition(accounts.nft as PublicKey);
      return nftEdition;
    }
  }),

  resolveIndividual(async ({ idlIx, path, accounts, programId }) => {
    if (
      path[path.length - 1] == "ownerTokenRecord" &&
      accounts.nft &&
      accounts.nftSource &&
      idlIx.name === "listNftV0"
    ) {
      const tokenMintRecord = await findTokenRecordPda(
        accounts.nft as PublicKey,
        accounts.nftSource as PublicKey
      );

      return tokenMintRecord;
    }
  }),
  resolveIndividual(async ({ idlIx, path, accounts, programId }) => {
    if (
      path[path.length - 1] == "ownerTokenRecord" &&
      accounts.nft &&
      accounts.nftEscrow &&
      idlIx.name === "executeSaleV0"
    ) {
      const tokenMintRecord = await findTokenRecordPda(
        accounts.nft as PublicKey,
        accounts.nftEscrow as PublicKey
      );

      return tokenMintRecord;
    }
  }),
  resolveIndividual(async ({ idlIx, path, accounts, programId }) => {
    if (
      path[path.length - 1] == "destinationTokenRecord" &&
      accounts.nft &&
      accounts.nftEscrow &&
      idlIx.name === "listNftV0"
    ) {
      const tokenMintRecord = await findTokenRecordPda(
        accounts.nft as PublicKey,
        accounts.nftEscrow as PublicKey
      );
      return tokenMintRecord;
    }
  }),

  resolveIndividual(async ({ idlIx, path, accounts, programId }) => {
    if (
      path[path.length - 1] == "destinationTokenRecord" &&
      accounts.nft &&
      accounts.nftRecipient &&
      idlIx.name === "executeSaleV0"
    ) {
      const tokenMintRecord = await findTokenRecordPda(
        accounts.nft as PublicKey,
        accounts.nftRecipient as PublicKey
      );
      return tokenMintRecord;
    }
  }),

  resolveIndividual(async ({ path }) => {
    if (path[path.length - 1] == "authorizationRules") {
      return MPL_DEFAULT_RULE_SET;
    }
  }),

  resolveIndividual(async ({ path }) => {
    if (path[path.length - 1] == "mplTokenAuthRulesProgram") {
      return new PublicKey("auth9SigNpDKz4sJJ1DfCTuZrZNSAgh9sFD3rboVmgg");
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
    owner: "owner",
  }),
  ataResolver({
    instruction: "claimReferralRewardsV0",
    account: "ownerTokenAccount",
    mint: "nft",
    owner: "owner",
  })
);
