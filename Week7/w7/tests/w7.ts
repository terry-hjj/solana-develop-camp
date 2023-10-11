
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { W7 } from "../target/types/w7";
import { PublicKey } from "@solana/web3.js";
import { expect } from "chai";

describe("w7", ()=>{
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.W7 as Program<W7>;

  it("create mint", async ()=>{
    let mintTokenAddr = new PublicKey("5jXVnYxQ89Yw5xjSxcKykFh2H8T78J7jFao56PhyG6u9");
    let info = {
      name: "abc",
      symbol: "def",
      icon: "icon",
    };

    const [mintMetaData, _] = await PublicKey.findProgramAddress(
      [
        new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").toBuffer(),
        mintTokenAddr.toBuffer(),
      ],
      program.programId,
    );

    await program.methods.extMint({
        name:"abc", symbol:"def", icon:"icon"
      }).accounts({
        user: provider.wallet.publicKey,
        mintMetaData: mintMetaData,
        systemProgram: new PublicKey("11111111111111111111111111111111"),
        tokenProgram: new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
        mintAccount: mintTokenAddr,
      }).rpc();
    
    expect((await program.account.mintMetaData.fetch(mintMetaData)).name).to.equal("abc");
  });
});


















































