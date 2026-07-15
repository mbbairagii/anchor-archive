#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

declare_id!("tMD3xni1voiHstU6MJPeTek2SxXA9EixKjNKhVvLNNW");

pub mod instructions;

pub use instructions::*;

#[program]
pub mod spl_tiny_token_lab {
    use super::*;

    pub fn create_mint(ctx: Context<CreateMint>, decimals: u8) -> Result<()> {
        instructions::create_mint::handler(ctx, decimals)
    }

    pub fn mint_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
        instructions::mint_tokens::handler(ctx, amount)
    }

    pub fn transfer_tokens(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {
        instructions::transfer_token::handler(ctx, amount)
    }
}