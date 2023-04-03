/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from "@metaplex-foundation/beet";
import * as web3 from "@solana/web3.js";

/**
 * @category Instructions
 * @category InsertOrAppend
 * @category generated
 */
export type InsertOrAppendInstructionArgs = {
  root: number[] /* size: 32 */;
  leaf: number[] /* size: 32 */;
  index: number;
};
/**
 * @category Instructions
 * @category InsertOrAppend
 * @category generated
 */
export const insertOrAppendStruct = new beet.BeetArgsStruct<
  InsertOrAppendInstructionArgs & {
    instructionDiscriminator: number[] /* size: 8 */;
  }
>(
  [
    ["instructionDiscriminator", beet.uniformFixedSizeArray(beet.u8, 8)],
    ["root", beet.uniformFixedSizeArray(beet.u8, 32)],
    ["leaf", beet.uniformFixedSizeArray(beet.u8, 32)],
    ["index", beet.u32],
  ],
  "InsertOrAppendInstructionArgs"
);
/**
 * Accounts required by the _insertOrAppend_ instruction
 *
 * @property [_writable_] merkleTree
 * @property [**signer**] authority
 * @property [] noop
 * @category Instructions
 * @category InsertOrAppend
 * @category generated
 */
export type InsertOrAppendInstructionAccounts = {
  merkleTree: web3.PublicKey;
  authority: web3.PublicKey;
  noop: web3.PublicKey;
  anchorRemainingAccounts?: web3.AccountMeta[];
};

export const insertOrAppendInstructionDiscriminator = [
  6, 42, 50, 190, 51, 109, 178, 168,
];

/**
 * Creates a _InsertOrAppend_ instruction.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @param args to provide as instruction data to the program
 *
 * @category Instructions
 * @category InsertOrAppend
 * @category generated
 */
export function createInsertOrAppendInstruction(
  accounts: InsertOrAppendInstructionAccounts,
  args: InsertOrAppendInstructionArgs,
  programId = new web3.PublicKey("cmtDvXumGCrqC1Age74AVPhSRVXJMd8PJS91L8KbNCK")
) {
  const [data] = insertOrAppendStruct.serialize({
    instructionDiscriminator: insertOrAppendInstructionDiscriminator,
    ...args,
  });
  const keys: web3.AccountMeta[] = [
    {
      pubkey: accounts.merkleTree,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.authority,
      isWritable: false,
      isSigner: true,
    },
    {
      pubkey: accounts.noop,
      isWritable: false,
      isSigner: false,
    },
  ];

  if (accounts.anchorRemainingAccounts != null) {
    for (const acc of accounts.anchorRemainingAccounts) {
      keys.push(acc);
    }
  }

  const ix = new web3.TransactionInstruction({
    programId,
    keys,
    data,
  });
  return ix;
}
