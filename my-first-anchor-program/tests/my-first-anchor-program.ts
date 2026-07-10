import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MyFirstAnchorProgram } from "../target/types/my_first_anchor_program";
import { BN } from "bn.js";

function circleIdToLeBytes(circleId: BN): Buffer {
  return circleId.toArrayLike(Buffer, "le", 8);
}

describe("my_first_anchor_program", () => {
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
      .accounts({ payer, myState: myStatePda, systemProgram: anchor.web3.SystemProgram.programId })
      .rpc();

    const account = await program.account.myState.fetch(myStatePda);
    if (!account.value.eq(initialValue)) throw new Error("initial value not set correctly");
    if (!account.owner.equals(payer)) throw new Error("owner not set correctly");
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
      .accounts({ payer, myState: myStatePda })
      .rpc();

    const account = await program.account.myState.fetch(myStatePda);
    if (!account.value.eq(newValue)) throw new Error("value not updated correctly");
  });

  it("creates circle #1", async () => {
    const owner = provider.wallet.publicKey;
    const circleId = new BN(1);
    const [circlePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("circle-state"), owner.toBuffer(), circleIdToLeBytes(circleId)],
      program.programId
    );
    const name = "Design Squad";

    await program.methods
      .createCircle(circleId, name)
      .accounts({ owner, circleState: circlePda, systemProgram: anchor.web3.SystemProgram.programId })
      .rpc();

    const circle = await program.account.circleState.fetch(circlePda);
    if (!circle.owner.equals(owner)) throw new Error("circle owner not set correctly");
    if (!circle.circleId.eq(circleId)) throw new Error("circle_id not set correctly");
    if (circle.name !== name) throw new Error("circle name not set correctly");
    if (!circle.memberCount.eq(new BN(0))) throw new Error("member_count should start at 0");
  });

  it("creates circle #2 for the SAME owner, proving multi-circle PDAs work", async () => {
    const owner = provider.wallet.publicKey;
    const circleId = new BN(2);
    const [circlePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("circle-state"), owner.toBuffer(), circleIdToLeBytes(circleId)],
      program.programId
    );
    const name = "Engineering Guild";

    await program.methods
      .createCircle(circleId, name)
      .accounts({ owner, circleState: circlePda, systemProgram: anchor.web3.SystemProgram.programId })
      .rpc();

    const circle = await program.account.circleState.fetch(circlePda);
    if (!circle.circleId.eq(circleId)) throw new Error("circle_id not set correctly for circle #2");
    if (circle.name !== name) throw new Error("circle name not set correctly for circle #2");
  });

  it("updates member count on circle #1 without affecting circle #2", async () => {
    const owner = provider.wallet.publicKey;
    const circleId1 = new BN(1);
    const circleId2 = new BN(2);

    const [circlePda1] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("circle-state"), owner.toBuffer(), circleIdToLeBytes(circleId1)],
      program.programId
    );
    const [circlePda2] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("circle-state"), owner.toBuffer(), circleIdToLeBytes(circleId2)],
      program.programId
    );

    const newMemberCount = new BN(10);

    await program.methods
      .updateMemberCount(circleId1, newMemberCount)
      .accounts({ owner, circleState: circlePda1 })
      .rpc();

    const circle1 = await program.account.circleState.fetch(circlePda1);
    const circle2 = await program.account.circleState.fetch(circlePda2);

    if (!circle1.memberCount.eq(newMemberCount)) throw new Error("member_count not updated correctly on circle #1");
    if (!circle2.memberCount.eq(new BN(0))) throw new Error("circle #2 member_count should remain untouched at 0");
  });

  it("rejects member count update from non-owner with ConstraintSeeds", async () => {
    const owner = provider.wallet.publicKey;
    const circleId = new BN(1);
    const [circlePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("circle-state"), owner.toBuffer(), circleIdToLeBytes(circleId)],
      program.programId
    );

    const intruder = anchor.web3.Keypair.generate();
    const airdropSig = await provider.connection.requestAirdrop(intruder.publicKey, anchor.web3.LAMPORTS_PER_SOL);
    await provider.connection.confirmTransaction(airdropSig);

    let didThrow = false;

    try {
      await program.methods
        .updateMemberCount(circleId, new BN(999))
        .accounts({ owner: intruder.publicKey, circleState: circlePda })
        .signers([intruder])
        .rpc();
    } catch (err) {
      didThrow = true;
      const anchorErr = err as anchor.AnchorError;
      if (anchorErr.error?.errorCode?.code !== "ConstraintSeeds") {
        throw new Error(`expected ConstraintSeeds but got ${anchorErr.error?.errorCode?.code}`);
      }
    }

    if (!didThrow) throw new Error("expected update_member_count to fail for non-owner, but it succeeded");
  });

    it("closes circle #2 and reclaims rent", async () => {
    const owner = provider.wallet.publicKey;
    const circleId = new BN(2);

    const [circlePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("circle-state"), owner.toBuffer(), circleIdToLeBytes(circleId)],
      program.programId
    );

    const balanceBefore = await provider.connection.getBalance(owner);

    await program.methods
      .closeCircle(circleId)
      .accounts({
        owner,
        circleState: circlePda,
      })
      .rpc();

    const balanceAfter = await provider.connection.getBalance(owner);

    console.log("Owner balance before close:", balanceBefore);
    console.log("Owner balance after close:", balanceAfter);

    const closedAccount = await provider.connection.getAccountInfo(circlePda);

    if (closedAccount !== null) {
      throw new Error("circle account should be closed, but it still exists");
    }

    if (balanceAfter <= balanceBefore) {
      throw new Error("owner should receive reclaimed rent after closing the circle");
    }
  });
});