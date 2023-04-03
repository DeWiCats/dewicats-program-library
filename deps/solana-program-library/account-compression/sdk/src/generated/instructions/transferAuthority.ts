/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as web3 from "@solana/web3.js";
import * as beetSolana from "@metaplex-foundation/beet-solana";
import * as beet from "@metaplex-foundation/beet";

/**
 * @category Instructions
 * @category TransferAuthority
 * @category generated
 */
export type TransferAuthorityInstructionArgs = {
  newAuthority: web3.PublicKey;
};
/**
 * @category Instructions
 * @category TransferAuthority
 * @category generated
 */
export const transferAuthorityStruct = new beet.BeetArgsStruct<
  TransferAuthorityInstructionArgs & {
    instructionDiscriminator: number[] /* size: 8 */;
  }
>(
  [
    ["instructionDiscriminator", beet.uniformFixedSizeArray(beet.u8, 8)],
    ["newAuthority", beetSolana.publicKey],
  ],
  "TransferAuthorityInstructionArgs"
);
/**
 * Accounts required by the _transferAuthority_ instruction
 *
 * @property [_writable_] merkleTree
 * @property [**signer**] authority
 * @category Instructions
 * @category TransferAuthority
 * @category generated
 */
export type TransferAuthorityInstructionAccounts = {
  merkleTree: web3.PublicKey;
  authority: web3.PublicKey;
  anchorRemainingAccounts?: web3.AccountMeta[];
};

export const transferAuthorityInstructionDiscriminator = [
  48, 169, 76, 72, 229, 180, 55, 161,
];

/**
 * Creates a _TransferAuthority_ instruction.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @param args to provide as instruction data to the program
 *
 * @category Instructions
 * @category TransferAuthority
 * @category generated
 */
export function createTransferAuthorityInstruction(
  accounts: TransferAuthorityInstructionAccounts,
  args: TransferAuthorityInstructionArgs,
  programId = new web3.PublicKey("cmtDvXumGCrqC1Age74AVPhSRVXJMd8PJS91L8KbNCK")
) {
  const [data] = transferAuthorityStruct.serialize({
    instructionDiscriminator: transferAuthorityInstructionDiscriminator,
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
