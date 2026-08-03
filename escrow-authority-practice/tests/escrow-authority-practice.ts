import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { assert } from "chai";
import { EscrowAuthorityPractice } from "../target/types/escrow_authority_practice";

describe("escrow-authority-practice", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .EscrowAuthorityPractice as Program<EscrowAuthorityPractice>;

  const maker = provider.wallet;

  const taker = anchor.web3.Keypair.generate();
  const mint = anchor.web3.Keypair.generate();

  const tradeId = new anchor.BN(1);
  const amount = new anchor.BN(100);

  function tradeIdBytes(id: anchor.BN) {
    return id.toArrayLike(Buffer, "le", 8);
  }

  function deriveEscrowPda(tradeId: anchor.BN) {
    return anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("escrow"),
        maker.publicKey.toBuffer(),
        taker.publicKey.toBuffer(),
        tradeIdBytes(tradeId),
      ],
      program.programId,
    );
  }

  function deriveAuthorityPda(escrow: anchor.web3.PublicKey) {
    return anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("escrow_authority"), escrow.toBuffer()],
      program.programId,
    );
  }

  it("initializes escrow and escrow authority", async () => {
    const [escrowPda, escrowBump] = deriveEscrowPda(tradeId);

    const [authorityPda, authorityBump] = deriveAuthorityPda(escrowPda);

    await program.methods
      .initEscrow(tradeId, escrowBump, authorityBump, amount)
      .accounts({
        escrow: escrowPda,
        escrowAuthority: authorityPda,
        maker: maker.publicKey,
        taker: taker.publicKey,
        mint: mint.publicKey,
      })
      .rpc();

    const escrow = await program.account.escrow.fetch(escrowPda);

    const authority = await program.account.escrowAuthority.fetch(authorityPda);

    assert.equal(escrow.maker.toBase58(), maker.publicKey.toBase58());

    assert.equal(escrow.taker.toBase58(), taker.publicKey.toBase58());

    assert.equal(escrow.mint.toBase58(), mint.publicKey.toBase58());

    assert.equal(escrow.tradeId.toNumber(), tradeId.toNumber());

    assert.equal(escrow.amount.toNumber(), amount.toNumber());

    assert.equal(escrow.bump, escrowBump);

    assert.equal(authority.escrow.toBase58(), escrowPda.toBase58());

    assert.equal(authority.bump, authorityBump);
  });

  it("validates escrow deposit with correct accounts", async () => {
    const [escrowPda] = deriveEscrowPda(tradeId);

    const [authorityPda] = deriveAuthorityPda(escrowPda);

    const vault = anchor.web3.Keypair.generate();

    await program.methods
      .depositToEscrow()
      .accounts({
        escrow: escrowPda,
        escrowAuthority: authorityPda,
        vault: vault.publicKey,
        maker: maker.publicKey,
        taker: taker.publicKey,
        mint: mint.publicKey,
      })
      .rpc();
  });

  it("rejects an incorrect escrow authority PDA", async () => {
    const [escrowPda] = deriveEscrowPda(tradeId);

    const fakeEscrow = anchor.web3.Keypair.generate().publicKey;

    const [wrongAuthority] = deriveAuthorityPda(fakeEscrow);

    const vault = anchor.web3.Keypair.generate();

    try {
      await program.methods
        .depositToEscrow()
        .accounts({
          escrow: escrowPda,
          escrowAuthority: wrongAuthority,
          vault: vault.publicKey,
          maker: maker.publicKey,
          taker: taker.publicKey,
          mint: mint.publicKey,
        })
        .rpc();

      assert.fail("Expected InvalidEscrow");
    } catch (err: any) {
      assert.include(err.toString(), "InvalidEscrow");
    }
  });

  it("rejects an incorrect mint", async () => {
    const [escrowPda] = deriveEscrowPda(tradeId);

    const [authorityPda] = deriveAuthorityPda(escrowPda);

    const vault = anchor.web3.Keypair.generate();

    const wrongMint = anchor.web3.Keypair.generate();

    try {
      await program.methods
        .depositToEscrow()
        .accounts({
          escrow: escrowPda,
          escrowAuthority: authorityPda,
          vault: vault.publicKey,
          maker: maker.publicKey,
          taker: taker.publicKey,
          mint: wrongMint.publicKey,
        })
        .rpc();

      assert.fail("Expected InvalidVaultMint");
    } catch (err: any) {
      assert.include(err.toString(), "InvalidVaultMint");
    }
  });

  it("derives different escrow PDAs for different trade ids", async () => {
    const [escrow1] = deriveEscrowPda(new anchor.BN(1));

    const [escrow2] = deriveEscrowPda(new anchor.BN(2));

    assert.notEqual(escrow1.toBase58(), escrow2.toBase58());
  });

  it("derives different escrow PDAs for different takers", async () => {
    const otherTaker = anchor.web3.Keypair.generate();

    const [escrow1] = deriveEscrowPda(tradeId);

    const [escrow2] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("escrow"),
        maker.publicKey.toBuffer(),
        otherTaker.publicKey.toBuffer(),
        tradeIdBytes(tradeId),
      ],
      program.programId,
    );

    assert.notEqual(escrow1.toBase58(), escrow2.toBase58());
  });
});
