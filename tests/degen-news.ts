import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { DegenNews } from "../target/types/degen_news";

describe("degen-news", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.DegenNews as Program<DegenNews>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
