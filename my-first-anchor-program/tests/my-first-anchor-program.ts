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

    // authority = real owner calling normally; owner = same key, used for PDA + has_one check
    await program.methods
      .updateMemberCount(newMemberCount)
      .accounts({
        authority: owner,
        owner: owner,
        circleState: circlePda,
      })
      .rpc();

    const circle = await program.account.circleState.fetch(circlePda);

    console.log("Updated Circle:", circle);

    if (!circle.memberCount.eq(newMemberCount)) {
      throw new Error("member_count not updated correctly");
    }
  });

  it("rejects member count update from non-owner with CircleError", async () => {
    const owner = provider.wallet.publicKey;

    const [circlePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("circle-state"), owner.toBuffer()],
      program.programId
    );

    const intruder = anchor.web3.Keypair.generate();

    const airdropSig = await provider.connection.requestAirdrop(
      intruder.publicKey,
      anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(airdropSig);

    let didThrow = false;

    try {
      // KNOWN GAP: intruder signs as `authority`, but passes the REAL owner's pubkey as `owner`.
      // has_one only checks circle_state.owner == owner.key(), which still passes here —
      // nothing currently checks that authority == owner. This call is expected to SUCCEED,
      // proving the current struct doesn't actually enforce real authorization yet.
      await program.methods
        .updateMemberCount(new BN(999))
        .accounts({
          authority: intruder.publicKey,
          owner: owner, // real owner's pubkey, satisfies has_one
          circleState: circlePda,
        })
        .signers([intruder])
        .rpc();
    } catch (err) {
      didThrow = true;

      const anchorErr = err as anchor.AnchorError;
      console.log("Caught expected error:", anchorErr.error?.errorCode);

      if (anchorErr.error?.errorCode?.code !== "UnauthorizedcircleUpdate") {
        throw new Error(
          `expected UnauthorizedcircleUpdate but got ${anchorErr.error?.errorCode?.code}`
        );
      }
    }

    if (!didThrow) {
      // this branch is what will actually happen right now — confirming the security gap
      throw new Error(
        "expected update_member_count to fail for non-owner, but it succeeded — authority is not actually enforced yet"
      );
    }
  });
});