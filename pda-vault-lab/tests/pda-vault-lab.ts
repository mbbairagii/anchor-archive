import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddressSync,
  getAccount,
} from "@solana/spl-token";

import { PdaVaultLab } from "../target/types/pda_vault_lab";

describe("pda-vault-lab", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.PdaVaultLab as Program<PdaVaultLab>;
  const signer = provider.wallet as anchor.Wallet;

  const mintAmount = new anchor.BN(1000);
  const depositAmount = new anchor.BN(400);
  const withdrawAmount = new anchor.BN(150);

  const [mintPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("mint")],
    program.programId
  );

  const [vaultAuthorityPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("vault")],
    program.programId
  );

  const [vaultTokenPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("vault-token")],
    program.programId
  );

  const userTokenAta = getAssociatedTokenAddressSync(
    mintPda,
    signer.publicKey,
    false,
    TOKEN_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM_ID
  );

  it("runs the full mint -> vault -> deposit -> withdraw flow", async () => {
    await program.methods
      .initializeMint()
      .accounts({
        signer: signer.publicKey,
        mint: mintPda,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    await program.methods
      .mintTokens(mintAmount)
      .accounts({
        signer: signer.publicKey,
        mint: mintPda,
        tokenAccount: userTokenAta,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    let userAccount = await getAccount(provider.connection, userTokenAta);
    console.log("User balance after mint:", Number(userAccount.amount));

    await program.methods
      .initializeVault()
      .accounts({
        signer: signer.publicKey,
        mint: mintPda,
        vaultAuthority: vaultAuthorityPda,
        vaultTokenAccount: vaultTokenPda,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    let vaultAccount = await getAccount(provider.connection, vaultTokenPda);
    console.log("Vault balance after init:", Number(vaultAccount.amount));

    await program.methods
      .deposit(depositAmount)
      .accounts({
        signer: signer.publicKey,
        mint: mintPda,
        userTokenAccount: userTokenAta,
        vaultTokenAccount: vaultTokenPda,
        vaultAuthority: vaultAuthorityPda,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    userAccount = await getAccount(provider.connection, userTokenAta);
    vaultAccount = await getAccount(provider.connection, vaultTokenPda);

    console.log("User balance after deposit:", Number(userAccount.amount));
    console.log("Vault balance after deposit:", Number(vaultAccount.amount));

    await program.methods
      .withdraw(withdrawAmount)
      .accounts({
        signer: signer.publicKey,
        mint: mintPda,
        userTokenAccount: userTokenAta,
        vaultTokenAccount: vaultTokenPda,
        vaultAuthority: vaultAuthorityPda,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    userAccount = await getAccount(provider.connection, userTokenAta);
    vaultAccount = await getAccount(provider.connection, vaultTokenPda);

    console.log("User balance after withdraw:", Number(userAccount.amount));
    console.log("Vault balance after withdraw:", Number(vaultAccount.amount));

    if (Number(userAccount.amount) !== 750) {
      throw new Error("User balance is incorrect after withdraw");
    }

    if (Number(vaultAccount.amount) !== 250) {
      throw new Error("Vault balance is incorrect after withdraw");
    }
  });
});