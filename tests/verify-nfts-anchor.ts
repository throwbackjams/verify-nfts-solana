import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { VerifyNftsAnchor } from "../target/types/verify_nfts_anchor";

describe("verify-nfts-anchor", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.VerifyNftsAnchor as Program<VerifyNftsAnchor>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
