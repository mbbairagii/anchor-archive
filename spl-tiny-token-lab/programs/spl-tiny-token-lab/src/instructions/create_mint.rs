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
//Why _ctx and _decimals?
//The underscore means “I’m intentionally not using this parameter right now.” It avoids unused variable warnings in Rust.



//this macro tells anchor how to validate and deserialize the instruction's acc before the handler runs
#[derive(Accounts)]
#[instruction(decimals: u8)]
pub struct CreateMint<'info> { //'info is a Rust lifetime used by Anchor for account references
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



//pipeline::
//You call create_mint(6).
//You pass:payer signer, fresh mint account address, token program, system program.
//Anchor reads CreateMint.
//Anchor sees init on mint, so it creates that account through the System Program.
//Anchor charges payer rent in lamports.
//Anchor initializes the new account as an SPL mint via the Token Program.
//Anchor stores: decimals = 6, mint authority = payer, freeze authority = payer.
//Then the handler runs and simply returns Ok(()).