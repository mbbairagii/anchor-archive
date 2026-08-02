#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

// Sharded fee counter practice : shards are just pdas with different seeds
//
// Goal:
// - Avoid a single hot global writable `fees` account.
// - Split fees across NUM_SHARDS PDAs.
// - Each user writes to one shard based on a deterministic shard_index.
//
// PDA layout for FeeShard:
//
// seeds = [
//     b"fee_shard",            // namespace
//     &[shard_index],          // domain: which shard (0..NUM_SHARDS-1)
// ]
//
// bump = canonical bump for this (program_id, seeds)
// Meaning: "fee shard number shard_index".

declare_id!("5iRu4NQR4W3Wua7ZnzrNpGcKqhbqgsv7vk4SwkZqbABh");

const NUM_SHARDS: u8 = 8; // configurable 8 shards

#[program]
pub mod sharded_fee_counter {
    use super::*;

    // Initialize a fee shard for given shard_index
    pub fn init_fee_shard(ctx: Context<InitFeeShard>, shard_index: u8, bump: u8) -> Result<()> {
        require!(shard_index < NUM_SHARDS, ErrorCode::InvalidShardIndex);

        let shard = &mut ctx.accounts.fee_shard;
        shard.shard_index = shard_index;
        shard.bump = bump;
        shard.total_fees = 0;
        Ok(())
    }

    // Increment fee in the appropriate shard
    pub fn add_fee(ctx: Context<AddFee>, shard_index: u8, amount: u64) -> Result<()> {
        require!(
            ctx.accounts.fee_shard.shard_index == shard_index,
            ErrorCode::ShardMismatch
        );

        let shard = &mut ctx.accounts.fee_shard;
        shard.total_fees = shard
            .total_fees
            .checked_add(amount)
            .ok_or(ErrorCode::Overflow)?;

        Ok(())
    }
}

#[account]
pub struct FeeShard {
    pub shard_index: u8,
    pub bump: u8,
    pub total_fees: u64,
}

// Init a single shard
#[derive(Accounts)]
#[instruction(shard_index: u8)]
pub struct InitFeeShard<'info> {
    #[account(
        init,
        seeds = [
            b"fee_shard",
            &[shard_index],
        ],
        bump,
        payer = payer,
        space = 8 + 1 + 1 + 8, // disc + shard_index + bump + total_fees
    )]
    pub fee_shard: Account<'info, FeeShard>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// Add fee to one shard
#[derive(Accounts)]
#[instruction(shard_index: u8, amount: u64)]
pub struct AddFee<'info> {
    #[account(
        mut,
        seeds = [
            b"fee_shard",
            &[shard_index],
        ],
        bump = fee_shard.bump,
    )]
    pub fee_shard: Account<'info, FeeShard>,

    #[account(mut)]
    pub payer: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid shard index")]
    InvalidShardIndex,

    #[msg("Shard index mismatch")]
    ShardMismatch,

    #[msg("Overflow in fee total")]
    Overflow,
}
