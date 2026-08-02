import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { assert } from "chai";
import { ShardedFeeCounter } from "../target/types/sharded_fee_counter";

describe("sharded-fee-counter", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .ShardedFeeCounter as Program<ShardedFeeCounter>;

  const payer = provider.wallet.publicKey;

  function deriveShardPda(shardIndex: number) {
    return anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("fee_shard"), Buffer.from([shardIndex])],
      program.programId,
    );
  }

  async function initShard(shardIndex: number) {
    const [shardPda, bump] = deriveShardPda(shardIndex);

    await program.methods
      .initFeeShard(shardIndex) // Use .initFeeShard(shardIndex, bump) if your instruction still accepts bump
      .accounts({
        feeShard: shardPda,
        payer,
      })
      .rpc();

    return { shardPda, bump };
  }

  it("initializes a fee shard", async () => {
    const shardIndex = 0;

    const { shardPda, bump } = await initShard(shardIndex);

    const shard = await program.account.feeShard.fetch(shardPda);

    assert.equal(shard.shardIndex, shardIndex);
    assert.equal(shard.bump, bump);
    assert.equal(shard.totalFees.toNumber(), 0);
  });

  it("adds fees to a shard", async () => {
    const shardIndex = 1;

    const { shardPda } = await initShard(shardIndex);

    await program.methods
      .addFee(shardIndex, new anchor.BN(25))
      .accounts({
        feeShard: shardPda,
        payer,
      })
      .rpc();

    const shard = await program.account.feeShard.fetch(shardPda);

    assert.equal(shard.totalFees.toNumber(), 25);
  });

  it("accumulates multiple fee additions", async () => {
    const shardIndex = 2;

    const { shardPda } = await initShard(shardIndex);

    await program.methods
      .addFee(shardIndex, new anchor.BN(20))
      .accounts({
        feeShard: shardPda,
        payer,
      })
      .rpc();

    await program.methods
      .addFee(shardIndex, new anchor.BN(30))
      .accounts({
        feeShard: shardPda,
        payer,
      })
      .rpc();

    const shard = await program.account.feeShard.fetch(shardPda);

    assert.equal(shard.totalFees.toNumber(), 50);
  });

  it("maintains independent totals across shards", async () => {
    const { shardPda: shard3 } = await initShard(3);
    const { shardPda: shard4 } = await initShard(4);

    await program.methods
      .addFee(3, new anchor.BN(50))
      .accounts({
        feeShard: shard3,
        payer,
      })
      .rpc();

    await program.methods
      .addFee(4, new anchor.BN(10))
      .accounts({
        feeShard: shard4,
        payer,
      })
      .rpc();

    const s3 = await program.account.feeShard.fetch(shard3);
    const s4 = await program.account.feeShard.fetch(shard4);

    assert.equal(s3.totalFees.toNumber(), 50);
    assert.equal(s4.totalFees.toNumber(), 10);
  });

  it("rejects an invalid shard index", async () => {
    const invalidShard = 8;

    const [shardPda] = deriveShardPda(invalidShard);

    try {
      await program.methods
        .initFeeShard(invalidShard)
        .accounts({
          feeShard: shardPda,
          payer,
        })
        .rpc();

      assert.fail("Expected InvalidShardIndex");
    } catch (err: any) {
      assert.include(err.toString(), "InvalidShardIndex");
    }
  });

  it("rejects an incorrect shard PDA", async () => {
    await initShard(5);

    const [wrongShard] = deriveShardPda(6);

    try {
      await program.methods
        .addFee(5, new anchor.BN(5))
        .accounts({
          feeShard: wrongShard,
          payer,
        })
        .rpc();

      assert.fail("Expected PDA seed constraint failure");
    } catch (err: any) {
      assert.include(err.toString(), "ConstraintSeeds");
    }
  });
});
