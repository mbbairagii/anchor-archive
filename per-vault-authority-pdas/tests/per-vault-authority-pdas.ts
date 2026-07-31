import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { assert } from "chai";
import { PerVaultAuthorityPdas } from "../target/types/per_vault_authority_pdas";

describe("per-vault-authority-pdas", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .PerVaultAuthorityPdas as Program<PerVaultAuthorityPdas>;

  const user = provider.wallet.publicKey;
  const marketId = new anchor.BN(1);

  function marketBytes(id: anchor.BN) {
    return id.toArrayLike(Buffer, "le", 8);
  }

  function deriveUserVault(owner: anchor.web3.PublicKey, market: anchor.BN) {
    return anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("user_vault"), owner.toBuffer(), marketBytes(market)],
      program.programId,
    );
  }

  function deriveVaultAuthority(vault: anchor.web3.PublicKey) {
    return anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault_authority"), vault.toBuffer()],
      program.programId,
    );
  }

  it("initializes a user-market vault", async () => {
    const [vaultPda, vaultBump] = deriveUserVault(user, marketId);

    await program.methods
      .initUserMarketVault(marketId, vaultBump)
      .accounts({
        userVault: vaultPda,
        user,
      })
      .rpc();

    const vault = await program.account.userMarketVault.fetch(vaultPda);

    assert.equal(vault.owner.toBase58(), user.toBase58());
    assert.equal(vault.marketId.toNumber(), marketId.toNumber());
    assert.equal(vault.bump, vaultBump);
    assert.equal(vault.balance.toNumber(), 0);
  });

  it("initializes a unique vault authority", async () => {
    const [vaultPda] = deriveUserVault(user, marketId);
    const [authorityPda, authorityBump] = deriveVaultAuthority(vaultPda);

    await program.methods
      .initVaultAuthority(marketId, authorityBump)
      .accounts({
        userVault: vaultPda,
        vaultAuthority: authorityPda,
        user,
      })
      .rpc();

    const authority = await program.account.vaultAuthority.fetch(authorityPda);

    assert.equal(authority.vault.toBase58(), vaultPda.toBase58());
    assert.equal(authority.bump, authorityBump);
  });

  it("creates different authority PDAs for different vaults", () => {
    const [vault1] = deriveUserVault(user, new anchor.BN(1));
    const [vault2] = deriveUserVault(user, new anchor.BN(2));

    const [authority1] = deriveVaultAuthority(vault1);
    const [authority2] = deriveVaultAuthority(vault2);

    assert.notEqual(vault1.toBase58(), vault2.toBase58());
    assert.notEqual(authority1.toBase58(), authority2.toBase58());
  });

  it("rejects an incorrect vault PDA", async () => {
    const wrongMarket = new anchor.BN(2);

    const [wrongVault, wrongBump] = deriveUserVault(user, wrongMarket);

    try {
      await program.methods
        .initUserMarketVault(marketId, wrongBump)
        .accounts({
          userVault: wrongVault,
          user,
        })
        .rpc();

      assert.fail("Expected PDA constraint failure");
    } catch (_) {
      // Expected: seeds mismatch.
    }
  });

  it("rejects an incorrect authority PDA", async () => {
    const [vaultPda] = deriveUserVault(user, marketId);

    const fakeVault = anchor.web3.Keypair.generate().publicKey;

    const [wrongAuthority, wrongBump] = deriveVaultAuthority(fakeVault);

    try {
      await program.methods
        .initVaultAuthority(marketId, wrongBump)
        .accounts({
          userVault: vaultPda,
          vaultAuthority: wrongAuthority,
          user,
        })
        .rpc();

      assert.fail("Expected PDA constraint failure");
    } catch (_) {
      // Expected: seeds mismatch.
    }
  });
});
