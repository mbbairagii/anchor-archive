use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token}; //mint is the account type wrapper for an spl mint account, token represents the spl token program itself

//This looks almost empty, and that is intentional.
//Why empty? Because the real work is done by Anchor through the account constraints:
//like : init, payer = payer, mint::decimals = decimals, etc
pub fn handler(_ctx: Context<CreateMint>, _decimals: u8) -> Result<()> {
    Ok(())
}

//this macro tells anchor how to validate and deserialize the instruction's acc before the handler runs
#[derive(Accounts)]
#[instruction(decimals: u8)]
pub struct CreateMint<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        mint::decimals = decimals,
        mint::authority = payer,
        mint::freeze_authority = payer,
    )]
    pub mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}