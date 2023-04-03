/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from "@metaplex-foundation/beet";

import {
  CompressionAccountType,
  compressionAccountTypeBeet,
} from "./CompressionAccountType";
import {
  ConcurrentMerkleTreeHeaderData,
  concurrentMerkleTreeHeaderDataBeet,
} from "./ConcurrentMerkleTreeHeaderData";
export type ConcurrentMerkleTreeHeader = {
  accountType: CompressionAccountType;
  header: ConcurrentMerkleTreeHeaderData;
};

/**
 * @category userTypes
 * @category generated
 */
export const concurrentMerkleTreeHeaderBeet =
  new beet.FixableBeetArgsStruct<ConcurrentMerkleTreeHeader>(
    [
      ["accountType", compressionAccountTypeBeet],
      ["header", concurrentMerkleTreeHeaderDataBeet],
    ],
    "ConcurrentMerkleTreeHeader"
  );
