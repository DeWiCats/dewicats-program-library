import {
  Commitment,
  Connection,
  Keypair,
  PublicKey,
  Signer,
  TransactionInstruction,
} from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import fetch from "node-fetch";
import {
  createAtaAndMintInstructions,
  createMintInstructions,
  sendInstructions,
  toBN,
} from "@helium/spl-utils";
import {
  createCreateMetadataAccountV3Instruction,
  PROGRAM_ID as METADATA_PROGRAM_ID,
} from "@metaplex-foundation/mpl-token-metadata";
import {
  AuthorityType,
  createSetAuthorityInstruction,
} from "@solana/spl-token";
import Squads from "@sqds/sdk";

export async function createAndMint({
  provider,
  mintKeypair = Keypair.generate(),
  amount,
  metadataUrl,
  decimals = 8,
  to,
  mintAuthority = provider.wallet.publicKey,
  freezeAuthority = provider.wallet.publicKey,
}: {
  provider: anchor.AnchorProvider;
  mintKeypair?: Keypair;
  amount: number;
  metadataUrl: string;
  decimals?: number;
  to?: PublicKey;
  mintAuthority?: PublicKey;
  freezeAuthority?: PublicKey;
}): Promise<void> {
  const mintTo = to || provider.wallet.publicKey;
  const metadata = await fetch(metadataUrl).then((r) => r.json());

  if (!(await exists(provider.connection, mintKeypair.publicKey))) {
    console.log(`${metadata.name} Mint not found, creating...`);
    await sendInstructions(
      provider,
      [
        ...(await createMintInstructions(
          provider,
          decimals,
          provider.wallet.publicKey,
          freezeAuthority,
          mintKeypair
        )),
        ...(
          await createAtaAndMintInstructions(
            provider,
            mintKeypair.publicKey,
            toBN(amount, decimals),
            mintTo
          )
        ).instructions,
      ],
      [mintKeypair]
    );
  }

  const metadataAddress = (
    await PublicKey.findProgramAddress(
      [
        Buffer.from("metadata", "utf-8"),
        METADATA_PROGRAM_ID.toBuffer(),
        mintKeypair.publicKey.toBuffer(),
      ],
      METADATA_PROGRAM_ID
    )
  )[0];

  if (!(await exists(provider.connection, metadataAddress))) {
    console.log(`${metadata.name} Metadata not found, creating...`);
    await sendInstructions(provider, [
      await createCreateMetadataAccountV3Instruction(
        {
          metadata: metadataAddress,
          mint: mintKeypair.publicKey,
          mintAuthority: provider.wallet.publicKey,
          payer: provider.wallet.publicKey,
          updateAuthority: provider.wallet.publicKey,
        },
        {
          createMetadataAccountArgsV3: {
            data: {
              name: metadata.name,
              symbol: metadata.symbol,
              uri: metadataUrl,
              sellerFeeBasisPoints: 0,
              creators: [
                {
                  address: provider.wallet.publicKey,
                  verified: true,
                  share: 100,
                },
              ],
              collection: null,
              uses: null,
            },
            isMutable: true,
            collectionDetails: null,
          },
        }
      ),
    ]);

    // Set mint authority to the proper authority
    if (!provider.wallet.publicKey.equals(mintAuthority)) {
      await sendInstructions(provider, [
        await createSetAuthorityInstruction(
          mintKeypair.publicKey,
          provider.wallet.publicKey,
          AuthorityType.MintTokens,
          mintAuthority
        ),
      ]);
    }
  }
}

export async function exists(
  connection: Connection,
  account: PublicKey
): Promise<boolean> {
  return Boolean(await connection.getAccountInfo(account));
}

export async function sendInstructionsOrSquads({
  provider,
  instructions,
  signers = [],
  payer = provider.wallet.publicKey,
  commitment = "confirmed",
  idlErrors = new Map(),
  executeTransaction = false,
  squads,
  multisig,
  authorityIndex,
}: {
  executeTransaction?: boolean; // Will execute the transaction immediately. Only works if the squads multisig is only 1 wallet threshold or signers is complete
  provider: anchor.AnchorProvider;
  instructions: TransactionInstruction[];
  signers?: Signer[];
  payer?: PublicKey;
  commitment?: Commitment;
  idlErrors?: Map<number, string>;
  squads: Squads;
  multisig?: PublicKey;
  authorityIndex: number;
}): Promise<void | undefined | string> {
  if (!multisig) {
    return await sendInstructions(
      provider,
      instructions,
      signers,
      payer,
      commitment,
      idlErrors
    );
  }

  const signerSet = new Set(
    instructions
      .map((ix) =>
        ix.keys.filter((k) => k.isSigner).map((k) => k.pubkey.toBase58())
      )
      .flat()
  );
  const signerKeys = Array.from(signerSet).map((k) => new PublicKey(k));

  const nonMissingSignerIxs = instructions.filter(
    (ix) =>
      !ix.keys.some(
        (k) => k.isSigner && !k.pubkey.equals(provider.wallet.publicKey)
      )
  );
  const squadsSignatures = signerKeys.filter(
    (k) =>
      !k.equals(provider.wallet.publicKey) &&
      !signers.some((s) => s.publicKey.equals(k))
  );

  if (squadsSignatures.length == 0) {
    return await sendInstructions(
      provider,
      nonMissingSignerIxs,
      signers,
      payer,
      commitment,
      idlErrors
    );
  }

  if (squadsSignatures.length >= 2) {
    throw new Error("Too many missing signatures");
  }

  const tx = await squads.createTransaction(multisig, authorityIndex);
  for (const ix of instructions) {
    await squads.addInstruction(tx.publicKey, ix);
  }

  await squads.activateTransaction(tx.publicKey);
  if (executeTransaction) {
    await squads.approveTransaction(tx.publicKey);
    const ix = await squads.buildExecuteTransaction(
      tx.publicKey,
      provider.wallet.publicKey
    );
    await sendInstructions(provider, [ix], signers);
  }
}
