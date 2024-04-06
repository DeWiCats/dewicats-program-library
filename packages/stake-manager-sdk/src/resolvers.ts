import {
  ataResolver,
  combineResolvers,
  heliumCommonResolver,
} from "@helium/spl-utils";

export const fanoutResolvers = combineResolvers(
  heliumCommonResolver,
  ataResolver({
    instruction: "initializeFanoutV0",
    account: "tokenAccount",
    mint: "fanoutMint",
    owner: "fanout",
  }),
  ataResolver({
    instruction: "initializeFanoutV0",
    account: "collectionAccount",
    mint: "collection",
    owner: "authority",
  }),
  ataResolver({
    instruction: "distributeV0",
    account: "receiptAccount",
    mint: "mint",
    owner: "owner",
  }),
  ataResolver({
    instruction: "distributeV0",
    account: "toAccount",
    mint: "fanoutMint",
    owner: "owner",
  })
);
