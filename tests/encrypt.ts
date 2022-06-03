import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Encrypt } from "../target/types/encrypt";

describe("encrypt", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Encrypt as Program<Encrypt>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
