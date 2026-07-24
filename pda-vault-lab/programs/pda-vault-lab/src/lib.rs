//goal of this entire prject:
//to learn that a pda can act as mint auth or vault auth
//the prog signs for it using seeds+bump
//and the token prog accepts the cpi only cuz the pda derivation matches

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{self, Mint, MintTo, TokenAccount, TokenInterface},
};

declare_id!("JCcPgN91g6u3zujTtS3ds45WPfDE1bXXuz7NXwfBRnhh");

#[program]
pub mod pda_vault_lab {
    use super::*;

    //this instruction deosnt jave any manual logiv inside the body cuz the real work is happening declaratively inside the InitializeMint acc
    //calling this func tells anchor to create and initialize the mint acc as specified by the acc constraints then return success
    pub fn initialize_mint(_ctx: Context<InitializeMint>) -> Result<()> {
        Ok(())
    }

    pub fn mint_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
        let signer_seeds: &[&[&[u8]]] = &[&[ //this is the most important line in the whole lesson. it rebuilds thr pda signer recipe using the same seed and the bump achor found for the mint pda
            b"mint",
            &[ctx.bumps.mint], // the bump that made thr pda valid and offcurve 
        ]];

        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.mint.to_account_info(), //this means that the mint pda is acting as the mint auth
        };

        let cpi_program = ctx.accounts.token_program.to_account_info(); //this gets the token prog acc info so anchor knows which external prog to call for the cpi
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts) //cpi context
            .with_signer(signer_seeds);

        token_interface::mint_to(cpi_ctx, amount)?; //mint call

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        mint::decimals = 6,
        mint::authority = mint,
        seeds = [b"mint"],
        bump
    )]
    pub mint: InterfaceAccount<'info, Mint>, //means this is an acc accessed through anchor's token interface abstraction which is useful when working ith token prog interfaces rather than plain anchor-owned acc

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"mint"],
        bump
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}





/*
pipeline:

Create a mint.

Set its mint authority to a PDA owned by your program.

Later, a user calls your program and asks to mint tokens.

Your program decides whether this request should be allowed.

If yes, your program calls the token program via CPI.

The runtime checks the PDA seeds and bump and temporarily lets that PDA count as a signer.

The token program sees a valid mint authority and mints the tokens.
*/