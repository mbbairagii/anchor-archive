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
}