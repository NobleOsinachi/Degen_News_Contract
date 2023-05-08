import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { CoodeAuctionRaffle } from "../target/types/coode_auction_raffle";

describe("coode-auction-raffle", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.CoodeAuctionRaffle as Program<CoodeAuctionRaffle>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
