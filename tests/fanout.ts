import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Fanout } from "@helium/idls/lib/types/fanout";
import {
  createAtaAndMint,
  createMint,
  createAtaAndTransfer,
} from "@helium/spl-utils";
import { Metaplex, walletAdapterIdentity } from "@metaplex-foundation/js";
import { getAccount, getAssociatedTokenAddressSync } from "@solana/spl-token";
import { ComputeBudgetProgram, PublicKey } from "@solana/web3.js";
import { expect } from "chai";
import { init, PROGRAM_ID } from "../packages/fanout-sdk";
import { createNFT } from "../utils/createNFT";
import { random } from "./utils/string";

describe("fanout", () => {
  anchor.setProvider(anchor.AnchorProvider.local("http://127.0.0.1:8899"));

  const provider = anchor.getProvider() as anchor.AnchorProvider;
  const me = provider.wallet.publicKey;
  const metaplex = new Metaplex(provider.connection);
  metaplex.use(walletAdapterIdentity(provider.wallet));

  let program: Program<Fanout>;
  let fanoutMint: PublicKey;
  let membershipMint: PublicKey;
  let fanoutName: string;
  let collection: PublicKey;

  beforeEach(async () => {
    let signature = await provider.connection.requestAirdrop(me, 1000000000);
    await provider.connection.confirmTransaction(signature);
    program = await init(provider, PROGRAM_ID, anchor.workspace.Fanout.idl);
    const { collectionAddress, nftAddress } = await createNFT(metaplex);
    console.log("collection", collectionAddress?.toBase58());
    console.log("nft", nftAddress?.toBase58());
    collection = collectionAddress!;
    membershipMint = nftAddress;
    fanoutName = random();
    fanoutMint = await createMint(provider, 0, me);
    // await createAtaAndMint(provider, membershipMint, 100);
    await createAtaAndMint(provider, fanoutMint, 100);
  });

  it("initializes a fanout", async () => {
    const {
      pubkeys: { fanout, tokenAccount },
    } = await program.methods
      .initializeFanoutV0({
        name: fanoutName,
      })
      .preInstructions([
        ComputeBudgetProgram.setComputeUnitLimit({ units: 500000 }),
      ])
      .accounts({
        collection,
        authority: provider.wallet.publicKey,
        membershipMint,
        fanoutMint,
      })
      .rpcAndKeys({ skipPreflight: true });

    const fanoutAcc = await program.account.fanoutV0.fetch(fanout!);
    expect(fanoutAcc.authority.toBase58()).to.eq(me.toBase58());
    expect(fanoutAcc.tokenAccount.toBase58()).to.eq(tokenAccount!.toBase58());
    expect(fanoutAcc.membershipCollection.toBase58()).to.eq(
      collection!.toBase58()
    );
    expect(fanoutAcc.totalShares.toNumber()).to.eq(1);
    expect(fanoutAcc.totalInflow.toNumber()).to.eq(0);
    expect(fanoutAcc.lastSnapshotAmount.toNumber()).to.eq(0);
    expect(fanoutAcc.name).to.eq(fanoutName);
  });

  describe("with fanout", () => {
    let fanout: PublicKey | undefined;
    let tokenAccount: PublicKey | undefined;
    beforeEach(async () => {
      ({
        pubkeys: { fanout, tokenAccount },
      } = await program.methods
        .initializeFanoutV0({
          name: fanoutName,
        })
        .preInstructions([
          ComputeBudgetProgram.setComputeUnitLimit({ units: 500000 }),
        ])
        .accounts({
          collection,
          authority: provider.wallet.publicKey,
          membershipMint,
          fanoutMint,
        })
        .rpcAndKeys({ skipPreflight: true }));
    });

    describe("with positions", () => {
      it("splits funds, accounting for dust", async () => {
        async function distribute() {
          await program.methods
            .distributeV0()
            .accounts({
              fanout,
              owner: me,
              mint: membershipMint,
            })
            .rpc({ skipPreflight: true });
        }

        await createAtaAndTransfer(provider, fanoutMint, 1, me, fanout);

        // TODO: Fix failure here
        await distribute();

        let toAccount = await getAccount(
          provider.connection,
          getAssociatedTokenAddressSync(fanoutMint, me)
        );
        // This first dist will ignore dust. Position 1 gets 0, position 2 gets 4.
        expect(toAccount.amount).to.eq(BigInt(Math.floor((1 / 100) * 4)));

        await createAtaAndTransfer(provider, fanoutMint, 1, me, fanout);

        await distribute();

        toAccount = await getAccount(
          provider.connection,
          getAssociatedTokenAddressSync(fanoutMint, me)
        );

        // Dust inclusive, should be a whole number
        expect(toAccount.amount).to.eq(BigInt((1 / 100) * 5));
      });
    });
  });
});
