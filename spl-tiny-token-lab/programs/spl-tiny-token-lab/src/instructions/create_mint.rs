use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token}; //mint is the account type wrapper for an spl mint account, token represents the spl token program itself

//This looks almost empty, and that is intentional.
//Why empty? Because the real work is done by Anchor through the account constraints:
//like : init, payer = payer, mint::decimals = decimals, etc
//Since Anchor already did the real setup from the constraints, the handler has nothing left to do.
//the reason this pert was even defined cuz every instruction must still have an entry point
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



//pipleine::
//You send create_mint(6).
//You pass in: payer wallet, a fresh mint account address, token program, system program
//Anchor sees #[account(init, ...)] on mint, so it creates that account.
//Anchor makes payer pay rent for that new account.
//Anchor initializes that new account as an SPL Mint account, not a normal custom account.
//Anchor stores: decimals = 6, mint authority = payer, freeze authority = payer
//Your handler returns Ok(()) because the important work is already done.