import {
  bundlrStorage,
  keypairIdentity,
  Metaplex,
} from "@metaplex-foundation/js";
import * as web3 from "@solana/web3.js";

export const createNFT = async (
  metaplex
): Promise<{
  nftAddress: web3.PublicKey;
  collectionAddress: web3.PublicKey;
}> => {
  console.log("hereee");

  const collectionAuthority = web3.Keypair.generate();
  const { nft: collectionNft } = await metaplex.nfts().create({
    name: "My Collection NFT",
    uri: "https://example.com/path/to/some/json/metadata.json",
    sellerFeeBasisPoints: 0,
    isCollection: true,
    collectionAuthority,
  });

  // console.log(`collectionNft`, collectionNft)

  const { nft } = await metaplex.nfts().create({
    uri: "URI",
    name: "NAME",
    symbol: "SYMBOL",
    sellerFeeBasisPoints: 0,
    collection: collectionNft.address,
  });

  return {
    nftAddress: nft.mint.address,
    collectionAddress: nft.collection?.address,
  };
};
