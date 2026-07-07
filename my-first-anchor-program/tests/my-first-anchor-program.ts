import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MyFirstAnchorProgram } from "../target/types/my_first_anchor_program";
import { BN } from "bn.js";

describe("my_first_anchor_program", () => {
  // Use the local validator
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  const program = anchor.workspace.MyFirstAnchorProgram as Program<MyFirstAnchorProgram>;

  it("initializes state", async () => {
    const payer = provider.wallet.publicKey;

    const [myStatePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("my-state"), payer.toBuffer()],
      program.programId
    );

    const initialValue = new BN(42);

    await program.methods
      .initialize(initialValue)
      .accounts({
        payer,
        myState: myStatePda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const account = await program.account.myState.fetch(myStatePda);

    console.log("Initialized Account:", account);

    if (!account.value.eq(initialValue)) {
      throw new Error("initial value not set correctly");
    }

    if (!account.owner.equals(payer)) {
      throw new Error("owner not set correctly");
    }
  });

  it("updates value", async () => {
    const payer = provider.wallet.publicKey;

    const [myStatePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("my-state"), payer.toBuffer()],
      program.programId
    );

    const newValue = new BN(100);

    await program.methods
      .updateValue(newValue)
      .accounts({
        payer,
        myState: myStatePda,
      })
      .rpc();

    const account = await program.account.myState.fetch(myStatePda);

    console.log("Updated Account:", account);

    if (!account.value.eq(newValue)) {
      throw new Error("value not updated correctly");
    }
  });
});