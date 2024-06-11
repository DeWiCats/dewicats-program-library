use anchor_lang::prelude::*;
use mpl_token_metadata::instruction::builders::TransferBuilder;
use mpl_token_metadata::instruction::InstructionBuilder;
use mpl_token_metadata::instruction::TransferArgs;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::{invoke, invoke_signed};

// Same transfer pnft utility just using program sign
// Able to use transfer from PDA to wallet
pub fn transfer_pnft_with_signer<'a>(
  source_token: AccountInfo<'a>,
  source_owner: AccountInfo<'a>,
  destination_token: AccountInfo<'a>,
  destination_owner: AccountInfo<'a>,
  payer: AccountInfo<'a>,
  authority: AccountInfo<'a>,
  mint: AccountInfo<'a>,
  metadata: AccountInfo<'a>,
  edition: AccountInfo<'a>,
  owner_token_record: AccountInfo<'a>,
  destination_token_record: AccountInfo<'a>,
  authorization_rules: AccountInfo<'a>,
  authorization_rules_program: AccountInfo<'a>,
  token_program: AccountInfo<'a>,
  token_metadata_program: AccountInfo<'a>,
  token_ata_program: AccountInfo<'a>,
  system_program: AccountInfo<'a>,
  sysvar_ins_program: AccountInfo<'a>,
  args: TransferArgs,
  // optional signers seed
  signers_seed: Option<&[&[&[u8]]; 1]>,
) -> ProgramResult {
  let mut builder = TransferBuilder::new();

  builder
    .token(source_token.key())
    .token_owner(source_owner.key())
    .destination(destination_token.key())
    .destination_owner(destination_owner.key())
    .mint(mint.key())
    .metadata(metadata.key())
    .edition(edition.key())
    .authority(authority.key())
    .payer(payer.key())
    .system_program(system_program.key())
    .sysvar_instructions(sysvar_ins_program.key())
    .spl_token_program(token_program.key())
    .spl_ata_program(token_ata_program.key())
    .authorization_rules_program(authorization_rules_program.key())
    .owner_token_record(owner_token_record.key())
    .destination_token_record(destination_token_record.key())
    .authorization_rules(authorization_rules.key());

  let transfer_ix = builder.build(args).unwrap().instruction();

  if let Some(signers_seed) = signers_seed {
    invoke_signed(
      &transfer_ix,
      &[
        source_token,
        source_owner,
        destination_token,
        destination_owner,
        mint,
        metadata,
        edition,
        owner_token_record,
        destination_token_record,
        authority,
        payer,
        system_program,
        sysvar_ins_program,
        token_program,
        token_ata_program,
        authorization_rules_program,
        authorization_rules,
        token_metadata_program,
      ],
      signers_seed,
    )
  } else {
    invoke(
      &transfer_ix,
      &[
        source_token,
        source_owner,
        destination_token,
        destination_owner,
        mint,
        metadata,
        edition,
        owner_token_record,
        destination_token_record,
        authority,
        payer,
        system_program,
        sysvar_ins_program,
        token_program,
        token_ata_program,
        authorization_rules_program,
        authorization_rules,
        token_metadata_program,
      ],
    )
  }
}
