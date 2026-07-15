import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SplTinyTokenLab } from "../target/types/spl_tiny_token_lab";
import {
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddressSync,
  getAccount,
  getMint,
} from "@solana/spl-token";
import { Keypair, SystemProgram, PublicKey } from "@solana/web3.js";
import assert from "assert";

describe("spl-tiny-token-lab", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .splTinyTokenLab as Program<SplTinyTokenLab>;

  it("creates a mint, mints tokens, and transfers between wallets", async () => {
    // 1. Create a fresh mint
    const mint = Keypair.generate();
    const decimals = 6;

    const payer = provider.wallet.publicKey;

    // Payer's ATA for this mint
    const payerTokenAccount = getAssociatedTokenAddressSync(
      mint.publicKey,
      payer
    );

    await program.methods
      .createMint(decimals)
      .accounts({
        payer,
        mint: mint.publicKey,
      })
      .signers([mint])
      .rpc();

    // 2. Mint tokens to payer's ATA
    const mintAmount = new anchor.BN(1_500_000);

    await program.methods
      .mintTokens(mintAmount)
      .accounts({
        authority: payer,
        mint: mint.publicKey,
        recipientTokenAccount: payerTokenAccount,
      })
      .rpc();

    // 3. Create a second wallet
    const recipient = Keypair.generate();

    // Recipient's ATA for this mint
    const recipientTokenAccount = getAssociatedTokenAddressSync(
      mint.publicKey,
      recipient.publicKey
    );

    // Fund the recipient with some SOL so it can create its ATA
    await provider.connection.requestAirdrop(
      recipient.publicKey,
      1_000_000_000
    );
    await new Promise((resolve) => setTimeout(resolve, 2000));

    const transferAmount = new anchor.BN(500_000);

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

    // 4. Verify balances
    const payerAfter = await getAccount(
      provider.connection,
      payerTokenAccount
    );
    const recipientAfter = await getAccount(
      provider.connection,
      recipientTokenAccount
    );

    const mintAccount = await getMint(provider.connection, mint.publicKey);

    // Payer minted 1.5, sent 0.5, so should have 1.0 left
    assert.strictEqual(Number(payerAfter.amount), 1_000_000);
    assert.strictEqual(Number(recipientAfter.amount), 500_000);

    // Mint total supply should be 1.5
    assert.strictEqual(Number(mintAccount.supply), 1_500_000);

    assert.strictEqual(mintAccount.decimals, decimals);

    // Both token accounts mint must be the same as our mint
    assert.strictEqual(
      payerAfter.mint.toBase58(),
      mint.publicKey.toBase58()
    );
    assert.strictEqual(
      recipientAfter.mint.toBase58(),
      mint.publicKey.toBase58()
    );
  });
});