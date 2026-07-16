import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SplTinyTokenLab } from "../target/types/spl_tiny_token_lab";
import {
  getAssociatedTokenAddressSync,
  getAccount,
  getMint,
} from "@solana/spl-token";
import { Keypair } from "@solana/web3.js";
import assert from "assert";

describe("spl-tiny-token-lab", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .splTinyTokenLab as Program<SplTinyTokenLab>;

  it("creates a mint, mints tokens, and transfers tokens", async () => {
    const payer = provider.wallet.publicKey;
    const recipient = Keypair.generate();
    const mint = Keypair.generate();

    const decimals = 6;
    const mintAmount = new anchor.BN(1_500_000);
    const transferAmount = new anchor.BN(500_000);

    const payerTokenAccount = getAssociatedTokenAddressSync(
      mint.publicKey,
      payer
    );

    const recipientTokenAccount = getAssociatedTokenAddressSync(
      mint.publicKey,
      recipient.publicKey
    );

    await program.methods
      .createMint(decimals)
      .accounts({
        payer,
        mint: mint.publicKey,
      })
      .signers([mint])
      .rpc();

    await program.methods
      .mintTokens(mintAmount)
      .accounts({
        authority: payer,
        mint: mint.publicKey,
        recipientTokenAccount: payerTokenAccount,
      })
      .rpc();

    await program.methods
      .transferTokens(transferAmount)
      .accounts({
        authority: payer,
        recipient: recipient.publicKey,
        mint: mint.publicKey,
        fromTokenAccount: payerTokenAccount,
        toTokenAccount: recipientTokenAccount,
      })
      .rpc();

    const mintAccount = await getMint(provider.connection, mint.publicKey);

    const payerAta = await getAccount(provider.connection, payerTokenAccount);
    const recipientAta = await getAccount(
      provider.connection,
      recipientTokenAccount
    );

    assert.strictEqual(mintAccount.decimals, decimals);
    assert.strictEqual(Number(mintAccount.supply), 1_500_000);

    assert.strictEqual(Number(payerAta.amount), 1_000_000);
    assert.strictEqual(Number(recipientAta.amount), 500_000);

    assert.strictEqual(payerAta.owner.toBase58(), payer.toBase58());
    assert.strictEqual(
      recipientAta.owner.toBase58(),
      recipient.publicKey.toBase58()
    );

    assert.strictEqual(payerAta.mint.toBase58(), mint.publicKey.toBase58());
    assert.strictEqual(recipientAta.mint.toBase58(), mint.publicKey.toBase58());
  });
});