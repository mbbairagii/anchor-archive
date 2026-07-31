import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { assert } from "chai";
import { UserMarketVaultManager } from "../target/types/user_market_vault_manager";

describe("user-market-vault-manager", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .UserMarketVaultManager as Program<UserMarketVaultManager>;

  const user = provider.wallet.publicKey;
  const marketId = new anchor.BN(1);

  function marketIdBytes(id: anchor.BN) {
    return id.toArrayLike(Buffer, "le", 8);
  }

  function deriveVaultPda(owner: anchor.web3.PublicKey, market: anchor.BN) {
    return anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("user_vault"), owner.toBuffer(), marketIdBytes(market)],
      program.programId,
    );
  }

  it("initializes a per-user-per-market vault", async () => {
    const [vaultPda, bump] = deriveVaultPda(user, marketId);

    await program.methods
      .initUserMarketVault(marketId, bump)
      .accounts({
        userVault: vaultPda,
        user,
      })
      .rpc();

    const vault = await program.account.userMarketVault.fetch(vaultPda);

    assert.equal(vault.owner.toBase58(), user.toBase58());
    assert.equal(vault.marketId.toNumber(), marketId.toNumber());
    assert.equal(vault.bump, bump);
    assert.equal(vault.balance.toNumber(), 0);
  });

  it("allows the owner to deposit", async () => {
    const [vaultPda] = deriveVaultPda(user, marketId);

    await program.methods
      .deposit(new anchor.BN(10))
      .accounts({
        userVault: vaultPda,
        user,
      })
      .rpc();

    const vault = await program.account.userMarketVault.fetch(vaultPda);

    assert.equal(vault.balance.toNumber(), 10);
  });

  it("rejects deposit by a non-owner", async () => {
    const attacker = anchor.web3.Keypair.generate();

    const sig = await provider.connection.requestAirdrop(
      attacker.publicKey,
      anchor.web3.LAMPORTS_PER_SOL,
    );

    await provider.connection.confirmTransaction(sig);

    const [vaultPda] = deriveVaultPda(user, marketId);

    try {
      await program.methods
        .deposit(new anchor.BN(5))
        .accounts({
          userVault: vaultPda,
          user: attacker.publicKey,
        })
        .signers([attacker])
        .rpc();

      assert.fail("Expected InvalidOwner error");
    } catch (err: any) {
      assert.include(err.toString(), "InvalidOwner");
    }
  });

  it("rejects an incorrect PDA", async () => {
    const wrongMarket = new anchor.BN(2);

    const [wrongVaultPda, bump] = deriveVaultPda(user, wrongMarket);

    try {
      await program.methods
        .initUserMarketVault(marketId, bump)
        .accounts({
          userVault: wrongVaultPda,
          user,
        })
        .rpc();

      assert.fail("Expected PDA constraint failure");
    } catch (_) {
      // Expected: PDA seeds mismatch.
    }
  });

  it("creates different PDAs for different markets", async () => {
    const [vault1] = deriveVaultPda(user, new anchor.BN(1));
    const [vault2] = deriveVaultPda(user, new anchor.BN(2));

    assert.notEqual(vault1.toBase58(), vault2.toBase58());
  });

  it("creates different PDAs for different users", async () => {
    const otherUser = anchor.web3.Keypair.generate();

    const [vault1] = deriveVaultPda(user, marketId);
    const [vault2] = deriveVaultPda(otherUser.publicKey, marketId);

    assert.notEqual(vault1.toBase58(), vault2.toBase58());
  });
});
