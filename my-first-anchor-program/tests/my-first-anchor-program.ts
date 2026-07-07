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

  it("creates circle", async () => {
    const owner = provider.wallet.publicKey;

    const [circlePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("circle-state"), owner.toBuffer()],
      program.programId
    );

    const circleId = new BN(1);
    const name = "Design Squad";

    await program.methods
      .createCircle(circleId, name)
      .accounts({
        owner,
        circleState: circlePda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const circle = await program.account.circleState.fetch(circlePda);

    console.log("Created Circle:", circle);

    if (!circle.owner.equals(owner)) {
      throw new Error("circle owner not set correctly");
    }

    if (!circle.circleId.eq(circleId)) {
      throw new Error("circle_id not set correctly");
    }

    if (circle.name !== name) {
      throw new Error("circle name not set correctly");
    }

    if (!circle.memberCount.eq(new BN(0))) {
      throw new Error("member_count should start at 0");
    }
  });

  it("updates member count", async () => {
    const owner = provider.wallet.publicKey;

    const [circlePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("circle-state"), owner.toBuffer()],
      program.programId
    );

    const newMemberCount = new BN(10);

    await program.methods
      .updateMemberCount(newMemberCount)
      .accounts({
        owner,
        circleState: circlePda,
      })
      .rpc();

    const circle = await program.account.circleState.fetch(circlePda);

    console.log("Updated Circle:", circle);

    if (!circle.memberCount.eq(newMemberCount)) {
      throw new Error("member_count not updated correctly");
    }
  });
});