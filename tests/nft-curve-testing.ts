import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { NftCurveTesting } from "../target/types/nft_curve_testing";

describe("nft-curve-testing", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.NftCurveTesting as Program<NftCurveTesting>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
