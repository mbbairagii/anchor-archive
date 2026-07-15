//in anchor, token transfers r usually done by making a cpi to the token prog's transfer_checked
//moving tokens from one token acc to another token acc of the same mint

//so, create_mint: define token
//mint_tokens: creste supply in an ATA
//transfer_tokens: move some of that supply to another wallet's ATA

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, TransferChecked},
};

pub fn handler(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {
    let decimals = ctx.accounts.mint.decimals; //reads mint's stored decimals Because transfer_checked needs the decimals value as an extra safety check.

    let cpi_accounts = TransferChecked {
        mint: ctx.accounts.mint.to_account_info(),
        from: ctx.accounts.from_token_account.to_account_info(),
        to: ctx.accounts.to_token_account.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };

    //Call the Token Program using these transfer accounts.
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
    );

    token::transfer_checked(cpi_ctx, amount, decimals)?; //main action which tells the token program to move amt from from_token_account to to_token_account, verify main context and decimals and the authority
    Ok(())
}

#[derive(Accounts)]
pub struct TransferTokens<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    pub recipient: SystemAccount<'info>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account( //from_token_account i.e. sender's ATA
        mut,
        associated_token::mint = mint,
        associated_token::authority = authority,
    )]
    pub from_token_account: Account<'info, TokenAccount>,

    #[account( //to_token_account i.e. destination ATA
        init_if_needed,
        payer = authority,
        associated_token::mint = mint,
        associated_token::authority = recipient,
    )]
    pub to_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}




//pipeline:
/*


-You call transfer_tokens(amount).
-You pass: sender authority signer,recipient wallet, mint, sender ATA, recipient ATA, token program, associated token program, system program.
-Anchor validates: authority signed, from_token_account is the ATA for (authority, mint), to_token_account is the ATA for (recipient, mint) or can be created that way.
-If the recipient ATA does not exist, Anchor creates it with authority paying rent.
-Handler reads mint.decimals.
-Handler builds TransferChecked CPI accounts.
-Handler CPI-calls the Token Program’s transfer_checked.
-Token Program verifies authority and decimals, then moves balance from sender ATA to recipient ATA.


*/