//workflow;
//This instruction will do two things in one call:
//create the recipient’s ATA if needed, mint some amount of your token into that ATA.

//So the mental flow becomes:
//create_mint = define token.
//mint_tokens = create actual units of that token and deposit them into a token account.

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, MintTo, Token, TokenAccount},
};

pub fn handler(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
    let cpi_accounts = MintTo { //This creates the account bundle required by the Token Program’s mint_to instruction.
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.recipient_token_account.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };

    let cpi_ctx = CpiContext::new( //This is a Cross Program Invocation (CPI), which means your program is calling another on-chain program instead of doing the token state update by itself.
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
    );

    token::mint_to(cpi_ctx, amount)?; //this is the key line, it tells token prog to increase the mint's total supply and credit amount into the destination acc and authorize it using the mnt auth signer
    Ok(())
}

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)] //signer mutable cuz Because this signer may pay to create the ATA if it does not already exist, so its lamports can change.
    pub authority: Signer<'info>,

    #[account(mut, mint::authority = authority)]
    pub mint: Account<'info, Mint>,

    #[account( //this is the destination token acc 
        init_if_needed, // means if ATA doesnt exist, create it, if already exists, just validate and continue
        payer = authority,
        associated_token::mint = mint,
        associated_token::authority = authority,
    )]
    pub recipient_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>, //needed for the cpi to mint_to
    pub associated_token_program: Program<'info, AssociatedToken>, 
    pub system_program: Program<'info, System>,
}






//pipleline
/*
 
-You call mint_tokens(amount).
-You pass: authority signer, mint account, recipient ATA,token program, associated token program, system program.
-Anchor validates that: authority signed, mint is mutable, mint.mint_authority == authority.
-Anchor checks whether recipient_token_account exists.
-If not, Anchor creates the ATA for (authority, mint) using the Associated Token Program, with authority paying rent.
-Your handler runs.
-The handler builds a MintTo CPI account struct.
-The handler CPI-calls the Token Program’s mint_to.
-The Token Program: increases mint supply, increases destination token account balance by amount

*/