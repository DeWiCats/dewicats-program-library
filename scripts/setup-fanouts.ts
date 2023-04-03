import { ClockworkProvider } from "@clockwork-xyz/sdk";
import * as anchor from "@coral-xyz/anchor";
import { fanoutKey, init } from "@dewicats/fanout-sdk";
import { HNT_MINT, sendInstructions } from "@helium/spl-utils";
import {
  createAssociatedTokenAccountIdempotentInstruction,
  getAssociatedTokenAddressSync,
} from "@solana/spl-token";
import { ComputeBudgetProgram, PublicKey } from "@solana/web3.js";
import { exists } from "../utils/distribute";
import { walletAdapterIdentity, Metaplex } from "@metaplex-foundation/js";

const creator = new PublicKey("88rM3PtPhumvEc9c4vKS9SEkeJJNSfTMUmzMQEgYBAVf");
const mobileFanoutName = "DEWICAT_MOBILE_FANOUT";

async function run() {
  anchor.setProvider(
    anchor.AnchorProvider.local(process.env.ANCHOR_PROVIDER_URL)
  );
  const provider = anchor.getProvider() as anchor.AnchorProvider;

  let authority = provider.wallet.publicKey;

  const multisig = null;
  // let multisig = argv.multisig ? new PublicKey(argv.multisig) : null;
  // const squads = Squads.endpoint(
  //   process.env.ANCHOR_PROVIDER_URL || '',
  //   provider.wallet
  // );
  // if (multisig) {
  //   authority = squads.getAuthorityPDA(multisig, argv.authorityIndex);
  // }

  const metaplex = Metaplex.make(provider.connection).use(
    walletAdapterIdentity(provider.wallet)
  );

  const dewicats = await metaplex.nfts().findAllByCreator({ creator });

  // Get all accounts holding dewicats
  const accounts = await Promise.all(
    dewicats.reduce(async (acc, dewicat) => {
      const { address } = dewicat;
      const largestAccts = await provider.connection.getTokenLargestAccounts(
        address
      );

      const largestAcct = largestAccts.value[0];
      const { address: largestAcctAddress, amount } = largestAcct;
      const account = await provider.connection.getAccountInfo(
        largestAcctAddress
      );
      const data = account?.data;
      const owner = data?.slice(0, 32);
      const ownerAddress = owner ? new PublicKey(owner) : null;

      if (!ownerAddress) {
        return acc;
      }

      return {
        ...acc,
        [dewicat.address.toBase58()]: ownerAddress.toBase58(),
      };
    }, {} as any)
  );

  const fanoutProgram = await init(provider);

  const fanout = fanoutKey(mobileFanoutName)[0];
  const hntAccount = await getAssociatedTokenAddressSync(
    HNT_MINT,
    fanout,
    true
  );
  console.log("Outputting hnt to", hntAccount.toBase58());

  if (!dewicats[0] || !dewicats[0].collection) {
    throw new Error("No collection found");
  }

  if (!(await exists(provider.connection, fanout))) {
    await fanoutProgram.methods
      .initializeFanoutV0({
        name: mobileFanoutName,
      })
      .preInstructions([
        ComputeBudgetProgram.setComputeUnitLimit({ units: 500000 }),
      ])
      .accounts({
        authority,
        membershipMint: dewicats[0].address,
        fanoutMint: HNT_MINT,
        collection: dewicats[0].collection.address,
      })
      .rpc({ skipPreflight: true });
  }

  const clockworkProvider = new ClockworkProvider(
    provider.wallet,
    provider.connection
  );

  for (const [mint, address] of Object.entries(accounts)) {
    // TODO: What is this?
    // if (!account.hst || account.hst === 0 || account.hst === "0") {
    //   continue;
    // }

    // 2️⃣  Define a trigger condition.
    const trigger = {
      cron: {
        schedule: "0 0 30 * * * *",
        skippable: true,
      },
    };

    // 3️⃣ Create the thread.
    const threadId = `${mobileFanoutName}-${mint.slice(0, 8)}`;
    const [thread] = threadKey(provider.wallet.publicKey, threadId);
    console.log("Thread ID", threadId, thread.toBase58());
    const memberHntAccount = await getAssociatedTokenAddressSync(
      HNT_MINT,
      address,
      true
    );
    if (!(await exists(provider.connection, memberHntAccount))) {
      await sendInstructions(provider, [
        createAssociatedTokenAccountIdempotentInstruction(
          provider.wallet.publicKey,
          memberHntAccount,
          address,
          HNT_MINT
        ),
      ]);
    }

    const distributeIx = await fanoutProgram.methods
      .distributeV0()
      .accounts({
        payer: new PublicKey("C1ockworkPayer11111111111111111111111111111"),
        fanout,
        owner: address,
        mint,
      })
      .instruction();

    if (!(await exists(provider.connection, thread))) {
      const tx = await clockworkProvider.threadCreate(
        provider.wallet.publicKey, // authority
        threadId, // id
        [distributeIx], // instructions
        trigger, // trigger
        anchor.web3.LAMPORTS_PER_SOL // amount
      );
    } else {
      await clockworkProvider.threadUpdate(provider.wallet.publicKey, thread, {
        trigger,
      });
    }
  }
}

run()
  .catch((err) => {
    console.error(err);
    process.exit(1);
  })
  .then(() => process.exit());

const CLOCKWORK_PID = new PublicKey(
  "CLoCKyJ6DXBJqqu2VWx9RLbgnwwR6BMHHuyasVmfMzBh"
);
export function threadKey(
  authority: PublicKey,
  threadId: string,
  programId: PublicKey = CLOCKWORK_PID
): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from("thread", "utf8"),
      authority.toBuffer(),
      Buffer.from(threadId, "utf8"),
    ],
    programId
  );
}
