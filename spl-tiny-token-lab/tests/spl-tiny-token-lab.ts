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
import { Keypair, SystemProgram } from "@solana/web3.js";
import assert from "assert";

describe("spl-tiny-token-lab", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .splTinyTokenLab as Program<SplTinyTokenLab>;

  it("creates a mint and mints tokens into the payer ATA", async () => {
    const mint = Keypair.generate();
    const decimals = 6;
    const amount = new anchor.BN(1_500_000);

    const payer = provider.wallet.publicKey;

    const recipientTokenAccount = getAssociatedTokenAddressSync(
      mint.publicKey,
      payer
    );

    await program.methods
      .createMint(decimals)
      .accounts({
        payer,
        mint: mint.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([mint])
      .rpc();

    await program.methods
      .mintTokens(amount)
      .accounts({
        authority: payer,
        mint: mint.publicKey,
        recipientTokenAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    const mintAccount = await getMint(provider.connection, mint.publicKey);
    const tokenAccount = await getAccount(
      provider.connection,
      recipientTokenAccount
    );

    assert.strictEqual(mintAccount.decimals, decimals);
    assert.strictEqual(Number(tokenAccount.amount), 1_500_000);
    assert.strictEqual(tokenAccount.mint.toBase58(), mint.publicKey.toBase58());
    assert.strictEqual(tokenAccount.owner.toBase58(), payer.toBase58());
  });
});