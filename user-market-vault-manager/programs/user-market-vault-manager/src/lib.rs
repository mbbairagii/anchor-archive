#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

// this small project is to prove my understanding of per-user-per-market pda design
// PDA layout for UserMarketVault:
//
// seeds = [
//     b"user_vault",                 // namespace
//     owner_pubkey.as_ref(),         // identity (user)
//     &market_id.to_le_bytes(),      // domain (market)
// ]
//
// bump = canonical bump from find_program_address
//
// Meaning: "vault for this (owner, market_id)".

declare_id!("3yeC5UeSpHABA9YNYtyCHh2fsu4Jz2dr6mkbcNudKwhf");

#[program]
pub mod user_market_vault_manager {
    use super::*;

    pub fn init_user_market_vault(
        ctx: Context<InitializeMarketVault>,
        market_id: u64,
        bump: u8,
    ) -> Result<()> {
        let vault = &mut ctx.accounts.user_vault;
        vault.owner = ctx.accounts.user.key();
        vault.market_id = market_id;
        vault.bump = bump; //*ctx.bumps.get("user_vault").unwrap()
        vault.balance = 0;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        require!(amount > 0, ErrorCode::InvalidAmount);
        let vault = &mut ctx.accounts.user_vault;

        vault.balance = vault
            .balance
            .checked_add(amount)
            .ok_or(ErrorCode::Overflow)?;
        Ok(())
    }
}

#[account]
pub struct UserMarketVault {
    pub owner: Pubkey,  // user key
    pub market_id: u64, // domain
    pub bump: u8,       // canonical bump
    pub balance: u64,   // balance
}

#[derive(Accounts)]
#[instruction(market_id: u64)]
pub struct InitializeMarketVault<'info> {
    #[account(
        init,
        seeds = [
            b"user_vault",
            user.key().as_ref(),
            &market_id.to_le_bytes(),
        ],
        bump,
        payer = user,
        space = 8 + 32 + 8 + 1 + 8, // discriminator + owner + market_id + bump + balance
    )]
    pub user_vault: Account<'info, UserMarketVault>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(
        mut,
        seeds = [
            b"user_vault",
            user.key().as_ref(),
            &user_vault.market_id.to_le_bytes(),
        ],
        bump = user_vault.bump,
        constraint = user_vault.owner == user.key() @ ErrorCode::InvalidOwner,
    )]
    pub user_vault: Account<'info, UserMarketVault>,

    #[account(mut)]
    pub user: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid owner")]
    InvalidOwner,

    #[msg("Balance overflow")]
    Overflow,

    #[msg("Deposit amount must be greater than zero")]
    InvalidAmount,
}
