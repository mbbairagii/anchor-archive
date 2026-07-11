import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";
import { AnchorModularPractice } from "../target/types/anchor_modular_practice";

describe("anchor-modular-practice", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .anchorModularPractice as Program<AnchorModularPractice>;

  const payer = provider.wallet.publicKey;

  const [myStatePda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("my-state"), payer.toBuffer()],
    program.programId
  );

  it("initializes the PDA", async () => {
    await program.methods
      .initialize(new anchor.BN(42))
      .accounts({
        payer,
        myState: myStatePda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const account = await program.account.myState.fetch(myStatePda);

    expect(account.owner.toBase58()).to.equal(payer.toBase58());
    expect(account.value.toNumber()).to.equal(42);
  });

  it("updates the stored value", async () => {
    await program.methods
      .updateValue(new anchor.BN(99))
      .accounts({
        payer,
        myState: myStatePda,
      })
      .rpc();

    const account = await program.account.myState.fetch(myStatePda);

    expect(account.value.toNumber()).to.equal(99);
    expect(account.owner.toBase58()).to.equal(payer.toBase58());
  });
});