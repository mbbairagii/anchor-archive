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

  function u64ToLeBytes(value: anchor.BN): Buffer {
    return value.toArrayLike(Buffer, "le", 8);
  }

  it("initializes my_state PDA", async () => {
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

  it("updates my_state value", async () => {
    await program.methods
      .updateValue(new anchor.BN(99))
      .accounts({
        payer,
        myState: myStatePda,
      })
      .rpc();

    const account = await program.account.myState.fetch(myStatePda);

    expect(account.owner.toBase58()).to.equal(payer.toBase58());
    expect(account.value.toNumber()).to.equal(99);
  });

  it("creates circle #1", async () => {
    const owner = provider.wallet.publicKey;
    const circleId = new anchor.BN(1);

    const [circlePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("circle-state"), owner.toBuffer(), u64ToLeBytes(circleId)],
      program.programId
    );

    await program.methods
      .createCircle(circleId, "Founders")
      .accounts({
        owner,
        circleState: circlePda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const account = await program.account.circleState.fetch(circlePda);

    expect(account.owner.toBase58()).to.equal(owner.toBase58());
    expect(account.circleId.toNumber()).to.equal(1);
    expect(account.memberCount.toNumber()).to.equal(1);
    expect(account.name).to.equal("Founders");
  });

  it("creates circle #2 for the same owner", async () => {
    const owner = provider.wallet.publicKey;
    const circleId = new anchor.BN(2);

    const [circlePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("circle-state"), owner.toBuffer(), u64ToLeBytes(circleId)],
      program.programId
    );

    await program.methods
      .createCircle(circleId, "Core Team")
      .accounts({
        owner,
        circleState: circlePda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const account = await program.account.circleState.fetch(circlePda);

    expect(account.owner.toBase58()).to.equal(owner.toBase58());
    expect(account.circleId.toNumber()).to.equal(2);
    expect(account.memberCount.toNumber()).to.equal(1);
    expect(account.name).to.equal("Core Team");
  });

    it("updates circle #1 member count", async () => {
    const owner = provider.wallet.publicKey;
    const circleId = new anchor.BN(1);

    const [circlePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("circle-state"), owner.toBuffer(), u64ToLeBytes(circleId)],
      program.programId
    );

    await program.methods
      .updateMemberCount(circleId, new anchor.BN(25))
      .accounts({
        owner,
        circleState: circlePda,
      })
      .rpc();

    const account = await program.account.circleState.fetch(circlePda);
    expect(account.memberCount.toNumber()).to.equal(25);
  });

  it("closes circle #2", async () => {
    const owner = provider.wallet.publicKey;
    const circleId = new anchor.BN(2);

    const [circlePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("circle-state"), owner.toBuffer(), u64ToLeBytes(circleId)],
      program.programId
    );

    await program.methods
      .closeCircle(circleId)
      .accounts({
        owner,
        circleState: circlePda,
      })
      .rpc();

    const closed = await program.account.circleState.fetchNullable(circlePda);
    expect(closed).to.equal(null);
  });
});