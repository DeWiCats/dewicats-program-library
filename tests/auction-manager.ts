import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
  Keypair,
  PublicKey,
  ComputeBudgetProgram,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";
import chai from "chai";
import {
  bidderRecieptKey,
  init as initAuctionManager,
  referralRecipientKey,
} from "../packages/auction-manager-sdk/src";
const { expect } = chai;
import chaiAsPromised from "chai-as-promised";
import { random } from "./utils/string";
import { AuctionManager } from "../target/types/auction_manager";
import { ensureAuctionIdl } from "./utils/fixtures";
import {
  Metaplex,
  toBigNumber,
  walletAdapterIdentity,
  SplTokenAmount,
  SplTokenCurrency,
} from "@metaplex-foundation/js";

chai.use(chaiAsPromised);

let auctionProgram: Program<AuctionManager>;
let collection: PublicKey;
let mint: PublicKey;
let auctionManager: PublicKey;

describe("auction-manager", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider() as anchor.AnchorProvider;
  const me = provider.wallet.publicKey;
  const metaplex = new Metaplex(provider.connection);
  metaplex.use(walletAdapterIdentity(provider.wallet));

  const collectionAuthority = Keypair.generate();

  before(async () => {
    collection = (
      await metaplex.nfts().create({
        uri: "https://example.com",
        name: "test",
        symbol: "test",
        sellerFeeBasisPoints: 0,
        updateAuthority: collectionAuthority,
        tokenOwner: collectionAuthority.publicKey,
        isCollection: true,
      })
    ).nft.address;

    mint = (
      await metaplex.nfts().create({
        uri: "https://example.com",
        name: "test",
        symbol: "test",
        sellerFeeBasisPoints: 0,
        collection,
        collectionAuthority,
      })
    ).nft.address;

    auctionProgram = await initAuctionManager(
      provider,
      anchor.workspace.auctionManager.programId,
      anchor.workspace.auctionManager.idl
    );
    ensureAuctionIdl(auctionProgram);
  });

  it("should initialize a auction manager", async () => {
    const name = random();

    const {
      pubkeys: { auctionManager: aM },
    } = await auctionProgram.methods
      .initializeManagerV0({
        name,
        collection,
        listingAuthority: me,
        updateAuthority: me,
        rewardPercentage: toBigNumber(30),
        auctionProceedsWallet: me,
      })
      .preInstructions([
        ComputeBudgetProgram.setComputeUnitLimit({ units: 500000 }),
      ])
      .accounts({
        collection,
      })
      .rpcAndKeys({ skipPreflight: true });

    auctionManager = aM!;

    const auctionManagerAcc =
      await auctionProgram.account.auctionManagerV0.fetch(auctionManager);

    expect(auctionManagerAcc.listingAuthority.toBase58()).to.eq(me.toBase58());
    expect(auctionManagerAcc.updateAuthority.toBase58()).to.eq(me.toBase58());
    expect(auctionManagerAcc.name).to.eq(name);
    expect(auctionManagerAcc.rewardPercentage.toNumber()).to.eq(30);
  });

  describe("with a auction manager", async () => {
    let tokenMint: PublicKey;
    let listing: PublicKey;
    const bidderKeypir = Keypair.generate();
    let bidderProvider: anchor.AnchorProvider;

    before(async () => {
      bidderProvider = new anchor.AnchorProvider(
        provider.connection,
        new anchor.Wallet(bidderKeypir),
        {
          skipPreflight: true,
        }
      );

      const currency: SplTokenCurrency = {
        symbol: "MOBILE",
        decimals: 6,
        namespace: "spl-token",
      };

      const initialSupply: SplTokenAmount = {
        basisPoints: toBigNumber(100000 * 100000),
        currency,
      };

      tokenMint = (
        await metaplex.tokens().createTokenWithMint({
          decimals: 6,
          freezeAuthority: me,
          initialSupply,
          mintAuthority: metaplex.identity(),
          owner: bidderKeypir.publicKey,
        })
      ).token.mint.address;

      await provider.connection.requestAirdrop(
        bidderKeypir.publicKey,
        LAMPORTS_PER_SOL
      );
    });

    it("allows to list an nft", async () => {
      const {
        pubkeys: { listing: l },
      } = await auctionProgram.methods
        .listNftV0({
          duration: toBigNumber(1000),
          startingPrice: toBigNumber(100),
        })
        .accounts({
          nft: mint,
          tokenMint,
          auctionManager: auctionManager!,
        })
        .rpcAndKeys({ skipPreflight: true });

      listing = l!;

      let listingAcc = await auctionProgram.account.listingV0.fetch(listing);

      expect(listingAcc.bidAmount.toNumber()).to.eq(0);
      expect(listingAcc.startingPrice.toNumber()).to.eq(100);
      expect(listingAcc.duration.toNumber()).to.eq(1000);
      expect(listingAcc.nft.toBase58()).to.eq(mint.toBase58());
      expect(listingAcc.tokenMint.toBase58()).to.eq(tokenMint.toBase58());
    });

    it("allows to bid on an nft", async () => {
      const initBidTxn = await auctionProgram.methods
        .initializeBidRecieptV0({
          amount: toBigNumber(0),
        })
        .accounts({
          listing,
          payer: bidderKeypir.publicKey,
        })
        .transaction();

      initBidTxn.feePayer = bidderKeypir.publicKey;
      initBidTxn.recentBlockhash = (
        await provider.connection.getLatestBlockhash()
      ).blockhash;

      const signedInitBidTxn = await bidderProvider.wallet.signTransaction(
        initBidTxn
      );

      await bidderProvider.sendAndConfirm(signedInitBidTxn);

      const bidReciept = bidderRecieptKey(listing, bidderKeypir.publicKey)[0];

      let bidRecieptAcc = await auctionProgram.account.bidRecieptV0.fetch(
        bidReciept
      );

      expect(bidRecieptAcc.amount.toNumber()).eq(0);

      const placeBidTxn = await auctionProgram.methods
        .placeBidV0({
          amount: toBigNumber(100000 * 5),
          referralCode: null,
        })
        .accounts({
          listing,
          tokenMint,
          auctionManager,
          bidReciept,
          payer: bidderKeypir.publicKey,
          referralRecipient: null,
        })
        .transaction();

      placeBidTxn.feePayer = bidderKeypir.publicKey;
      placeBidTxn.recentBlockhash = (
        await provider.connection.getLatestBlockhash()
      ).blockhash;

      const signedPlaceBidTxn = await bidderProvider.wallet.signTransaction(
        placeBidTxn
      );

      await bidderProvider.sendAndConfirm(signedPlaceBidTxn);

      let newBidRecieptAcc = await auctionProgram.account.bidRecieptV0.fetch(
        bidReciept
      );

      expect(newBidRecieptAcc.bidder.toBase58()).to.eq(
        bidderKeypir.publicKey.toBase58()
      );
      expect(newBidRecieptAcc.amount.toNumber()).to.eq(100000 * 5);
      expect(newBidRecieptAcc.listing.toBase58()).to.eq(listing.toBase58());
    });

    it("allows to initialize referral recipient", async () => {
      const {
        pubkeys: { referralRecipient },
      } = await auctionProgram.methods
        .initializeReferralRecipientV0({
          referralCode: "PERONI",
        })
        .accounts({
          listing,
          auctionManager,
          nft: mint,
        })
        .rpcAndKeys({ skipPreflight: true });

      if (!referralRecipient) {
        throw new Error("Referral recipient not found");
      }

      const referralRecipientAcc =
        await auctionProgram.account.referralRecipientV0.fetch(
          referralRecipient
        );

      expect(referralRecipientAcc.referralCode).to.eq("PERONI");
      expect(referralRecipientAcc.nft.toBase58()).to.eq(mint.toBase58());
      expect(referralRecipientAcc.count.toNumber()).to.eq(0);
      expect(referralRecipientAcc.claimed).to.eq(false);
    });

    it("allows to bid on nft with referral", async () => {
      const bidReciept = bidderRecieptKey(listing, bidderKeypir.publicKey)[0];

      const placeBidTxn = await auctionProgram.methods
        .placeBidV0({
          amount: toBigNumber(100000 * 8),
          referralCode: "PERONI",
        })
        .accounts({
          listing,
          tokenMint,
          auctionManager,
          bidReciept,
          payer: bidderKeypir.publicKey,
          referralRecipient: referralRecipientKey(listing, mint)[0],
        })
        .transaction();

      placeBidTxn.feePayer = bidderKeypir.publicKey;
      placeBidTxn.recentBlockhash = (
        await provider.connection.getLatestBlockhash()
      ).blockhash;

      const signedPlaceBidTxn = await bidderProvider.wallet.signTransaction(
        placeBidTxn
      );

      await bidderProvider.sendAndConfirm(signedPlaceBidTxn);

      const newBidRecieptAcc = await auctionProgram.account.bidRecieptV0.fetch(
        bidReciept
      );

      const listingAcc = await auctionProgram.account.listingV0.fetch(listing);

      const referralRecipientAcc =
        await auctionProgram.account.referralRecipientV0.fetch(
          referralRecipientKey(listing, mint)[0]
        );

      expect(listingAcc.totalReferralCount.toNumber()).to.eq(1);
      expect(newBidRecieptAcc.bidder.toBase58()).to.eq(
        bidderKeypir.publicKey.toBase58()
      );
      expect(newBidRecieptAcc.amount.toNumber()).to.eq(100000 * 8);
      expect(newBidRecieptAcc.listing.toBase58()).to.eq(listing.toBase58());
      expect(newBidRecieptAcc.referralRecipient?.toBase58()).to.eq(
        referralRecipientKey(listing, mint)[0].toBase58()
      );
      expect(referralRecipientAcc.count.toNumber()).to.eq(1);
    });

    it("allows to cancel bid", async () => {
      const bidReciept = bidderRecieptKey(listing, bidderKeypir.publicKey)[0];

      const placeBidTxn = await auctionProgram.methods
        .placeBidV0({
          amount: toBigNumber(100000 * 10),
          referralCode: null,
        })
        .accounts({
          listing,
          tokenMint,
          auctionManager,
          bidReciept,
          payer: bidderKeypir.publicKey,
          referralRecipient: null,
        })
        .transaction();

      placeBidTxn.feePayer = bidderKeypir.publicKey;
      placeBidTxn.recentBlockhash = (
        await provider.connection.getLatestBlockhash()
      ).blockhash;

      const signedPlaceBidTxn = await bidderProvider.wallet.signTransaction(
        placeBidTxn
      );

      await bidderProvider.sendAndConfirm(signedPlaceBidTxn);

      let newBidRecieptAcc = await auctionProgram.account.bidRecieptV0.fetch(
        bidReciept
      );

      expect(newBidRecieptAcc.bidder.toBase58()).to.eq(
        bidderKeypir.publicKey.toBase58()
      );
      expect(newBidRecieptAcc.amount.toNumber()).to.eq(100000 * 10);
      expect(newBidRecieptAcc.listing.toBase58()).to.eq(listing.toBase58());

      const cancelBidTxn = await auctionProgram.methods
        .cancelBidV0({
          bidReciept,
        })
        .accounts({
          listing,
          bidReciept,
          payer: bidderKeypir.publicKey,
          tokenMint,
          auctionManager,
        })
        .transaction();

      cancelBidTxn.feePayer = bidderKeypir.publicKey;
      cancelBidTxn.recentBlockhash = (
        await provider.connection.getLatestBlockhash()
      ).blockhash;

      const signedCancelBidTxn = await bidderProvider.wallet.signTransaction(
        cancelBidTxn
      );

      await bidderProvider.sendAndConfirm(signedCancelBidTxn);

      let cancelledBidRecieptAcc =
        await auctionProgram.account.bidRecieptV0.fetch(bidReciept);

      expect(cancelledBidRecieptAcc.bidder.toBase58()).to.eq(
        bidderKeypir.publicKey.toBase58()
      );
      expect(cancelledBidRecieptAcc.amount.toNumber()).to.eq(100000 * 10);
      expect(cancelledBidRecieptAcc.listing.toBase58()).to.eq(
        listing.toBase58()
      );
      expect(cancelledBidRecieptAcc.state.cancelled).to.exist;
      expect(cancelledBidRecieptAcc.state?.active).to.not.exist;
    });

    it("allows to execute sale", async () => {
      const bidReciept = bidderRecieptKey(listing, bidderKeypir.publicKey)[0];

      const placeBidTxn = await auctionProgram.methods
        .placeBidV0({
          amount: toBigNumber(100000 * 20),
          referralCode: null,
        })
        .accounts({
          listing,
          tokenMint,
          auctionManager,
          bidReciept,
          payer: bidderKeypir.publicKey,
          referralRecipient: null,
        })
        .transaction();

      placeBidTxn.feePayer = bidderKeypir.publicKey;
      placeBidTxn.recentBlockhash = (
        await provider.connection.getLatestBlockhash()
      ).blockhash;

      const signedPlaceBidTxn = await bidderProvider.wallet.signTransaction(
        placeBidTxn
      );

      await bidderProvider.sendAndConfirm(signedPlaceBidTxn);

      let newBidRecieptAcc = await auctionProgram.account.bidRecieptV0.fetch(
        bidReciept
      );

      expect(newBidRecieptAcc.bidder.toBase58()).to.eq(
        bidderKeypir.publicKey.toBase58()
      );
      expect(newBidRecieptAcc.amount.toNumber()).to.eq(100000 * 20);
      expect(newBidRecieptAcc.listing.toBase58()).to.eq(listing.toBase58());

      await auctionProgram.methods
        .executeSaleV0({
          claimedRewards: false,
        })
        .accounts({
          auctionManager,
          auctionProceedsWallet: me,
          listing,
          bidder: bidderKeypir.publicKey,
          nft: mint,
          tokenMint,
          highestBidReciept: bidReciept,
          collection,
        })
        .rpcAndKeys({ skipPreflight: true });

      const executedSaleAcct = await auctionProgram.account.bidRecieptV0.fetch(
        bidReciept
      );

      expect(executedSaleAcct.state?.executed).to.exist;
      expect(executedSaleAcct.state?.active).to.not.exist;
    });

    it("allows to claim rewards", async () => {
      await auctionProgram.methods
        .claimReferralRewardsV0({
          recipientWallet: me,
        })
        .accounts({
          auctionManager,
          bidReciept: bidderRecieptKey(listing, bidderKeypir.publicKey)[0],
          referralRecipient: referralRecipientKey(listing, mint)[0],
          listing,
          tokenMint,
        })
        .rpcAndKeys({ skipPreflight: true });

      const referralRecipientAcc =
        await auctionProgram.account.referralRecipientV0.fetch(
          referralRecipientKey(listing, mint)[0]
        );

      expect(referralRecipientAcc.claimed).to.eq(true);
    });
  });
});
