//this is a small project to depen my undrstanding of per vault authority pdas so that each spl token vault can have its own unique pda so that no actual clashes happen

#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

// PDA layout for UserMarketVault:
//
// seeds = [
//     b"user_vault",                 // namespace
//     owner_pubkey.as_ref(),         // identity (user)
//     &market_id.to_le_bytes(),      // domain (market)
// ]
//
// Meaning: "vault for this (owner, market_id)".

// PDA layout for VaultAuthority:
//
// seeds = [
//     b"vault_authority",            // namespace
//     user_vault_pda.as_ref(),       // domain: specific vault
// ]
//
// Meaning: "authority for this exact user_vault PDA".

declare_id!("DKiB1H3Q1bVsXi1rk74KMgy2WSbUukUuLf32aUetpaMZ");

#[program]
pub mod per_vault_authority_pdas {
    use super::*;

    pub fn init_user_market_vault(ctx: Context<InitializeMarketVault>, market_id: u64, bump: u8) -> Result<()> {
        let vault=&mut ctx.accounts.user_vault;
        vault.owner=ctx.accounts.user.key();
        vault.market_id=market_id;
        vault.bump=bump;
        vault.balance=0;
        Ok(())
    }

    pub fn init_vault_authority(ctx: Context<InitVaultAuthority>, _market_id: u64, bump: u8) -> Result<()> {
        let auth = &mut ctx.accounts.vault_authority;
        auth.vault = ctx.accounts.user_vault.key();
        auth.bump = bump;
        Ok(())
    }
}

#[account]
pub struct UserMarketVault {
    pub owner: Pubkey,  // user
    pub market_id: u64, // domain
    pub bump: u8,       // vault PDA bump
    pub balance: u64,   // toy balance
}

#[account]
pub struct VaultAuthority {
    pub vault: Pubkey, // which UserMarketVault this authority belongs to
    pub bump: u8,      // authority PDA bump
}

// Init vault (per-user-per-market)
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
        space = 8 + 32 + 8 + 1 + 8, // disc + owner + market_id + bump + balance
    )]
    pub user_vault: Account<'info, UserMarketVault>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// Init vault authority (per-vault authority PDA)
#[derive(Accounts)]
#[instruction(market_id: u64)]
pub struct InitVaultAuthority<'info> {
    // Re-derive the vault PDA to ensure it matches the (user, market_id) pair
    #[account(
        mut,
        seeds = [
            b"user_vault",
            user.key().as_ref(),
            &market_id.to_le_bytes(),
        ],
        bump = user_vault.bump,
    )]
    pub user_vault: Account<'info, UserMarketVault>,

    #[account(
        init,
        seeds = [
            b"vault_authority",
            user_vault.key().as_ref(),
        ],
        bump,
        payer = user,
        space = 8 + 32 + 1, // disc + vault pubkey + bump
    )]
    pub vault_authority: Account<'info, VaultAuthority>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}
