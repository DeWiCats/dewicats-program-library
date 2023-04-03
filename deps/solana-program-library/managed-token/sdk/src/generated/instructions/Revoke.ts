/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as splToken from "@solana/spl-token";
import * as beet from "@metaplex-foundation/beet";
import * as web3 from "@solana/web3.js";

/**
 * @category Instructions
 * @category Revoke
 * @category generated
 */
export const RevokeStruct = new beet.BeetArgsStruct<{
  instructionDiscriminator: number;
}>([["instructionDiscriminator", beet.u8]], "RevokeInstructionArgs");
/**
 * Accounts required by the _Revoke_ instruction
 *
 * @property [] mint
 * @property [_writable_] account
 * @property [**signer**] owner
 * @property [**signer**] upstreamAuthority
 * @property [] freezeAuthority
 * @category Instructions
 * @category Revoke
 * @category generated
 */
export type RevokeInstructionAccounts = {
  mint: web3.PublicKey;
  account: web3.PublicKey;
  owner: web3.PublicKey;
  upstreamAuthority: web3.PublicKey;
  freezeAuthority: web3.PublicKey;
  tokenProgram?: web3.PublicKey;
};

export const revokeInstructionDiscriminator = 7;

/**
 * Creates a _Revoke_ instruction.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @category Instructions
 * @category Revoke
 * @category generated
 */
export function createRevokeInstruction(
  accounts: RevokeInstructionAccounts,
  programId = new web3.PublicKey("mTok58Lg4YfcmwqyrDHpf7ogp599WRhzb6PxjaBqAxS")
) {
  const [data] = RevokeStruct.serialize({
    instructionDiscriminator: revokeInstructionDiscriminator,
  });
  const keys: web3.AccountMeta[] = [
    {
      pubkey: accounts.mint,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.account,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.owner,
      isWritable: false,
      isSigner: true,
    },
    {
      pubkey: accounts.upstreamAuthority,
      isWritable: false,
      isSigner: true,
    },
    {
      pubkey: accounts.freezeAuthority,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.tokenProgram ?? splToken.TOKEN_PROGRAM_ID,
      isWritable: false,
      isSigner: false,
    },
  ];

  const ix = new web3.TransactionInstruction({
    programId,
    keys,
    data,
  });
  return ix;
}
