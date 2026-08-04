#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

//escrow pda +vault pda owned by pda (+a bit of acc validation as security)
//escrow pda:holds metadata for a pending transfer: sender, receiver, amount, token mint, expiry, etc
//seeds = [b"escrow", maker, taker, &trade_id.to_le_bytes()]
//
//vault pda owned by pda: spl token acc where the funds sit during escrow, its authority is a pda derived form the escrow pda
//seeds = [b"escrow_authority", escrow_pda.key().as_ref()]

declare_id!("BVjp8C7w2NhMEqrXdUaDdXi84ckrEu9chEynC1kjs9MD");

#[program]
pub mod escrow_authority_practice {
    use super::*;

    pub fn init_escrow(
        ctx: Context<InitEscrow>,
        trade_id: u64,
        escrow_bump: u8,
        auth_bump: u8,
        amount_a: u64,
        amount_b: u64,
    ) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;
        escrow.maker = ctx.accounts.maker.key();
        escrow.taker = ctx.accounts.taker.key();
        escrow.mint_a = ctx.accounts.mint_a.key();
        escrow.mint_b = ctx.accounts.mint_b.key();

        escrow.amount_a = amount_a;
        escrow.amount_b = amount_b;
        escrow.trade_id = trade_id;
        escrow.bump = escrow_bump;

        let auth = &mut ctx.accounts.escrow_authority;
        auth.escrow = escrow.key();
        auth.bump = auth_bump;

        Ok(())
    }

    pub fn deposit_to_escrow(ctx: Context<DepositToEscrow>) -> Result<()> {
        // For now, just validate relationships; SPL token CPIs come later.

        // Check that vault authority == escrow_authority.key() (once vault is a TokenAccount)
        // and that mint matches escrow.mint.
        // Placeholder sanity checks:
        require!(
            ctx.accounts.mint_a.key() == ctx.accounts.escrow.mint_a,
            ErrorCode::InvalidVaultMint
        );

        // You can’t check vault.owner here until vault is a TokenAccount,
        // but conceptually:
        // require!(vault.owner == escrow_authority.key(), ErrorCode::InvalidVaultAuthority);

        Ok(())
    }

    pub fn take_escrow(ctx: Context<TakeEscrow>) -> Result<()> {
        let escrow = &ctx.accounts.escrow;

        // 1) Taker pays Token B to maker
        pay_taker_to_maker(ctx.accounts.token_program.to_account_info(), &ctx, escrow)?;

        // 2) Vault sends Token A to taker (PDA authority)
        deliver_vault_to_taker(ctx.accounts.token_program.to_account_info(), &ctx, escrow)?;

        // 3) Optional: close escrow / vault (we can add this later)
        Ok(())
    }
}

fn deliver_vault_to_taker(
    token_program: AccountInfo,
    ctx: &Context<TakeEscrow>,
    escrow: &Escrow,
) -> Result<()> {
    let cpi_accounts = TransferChecked {
        from: ctx.accounts.vault_ata_a.to_account_info(),
        mint: ctx.accounts.mint_a.to_account_info(),
        to: ctx.accounts.taker_ata_a.to_account_info(),
        authority: ctx.accounts.escrow_authority.to_account_info(),
    };

    // Seeds for escrow_authority PDA: must match the Accounts struct
    let seeds: &[&[u8]] = &[
        b"escrow_authority",
        ctx.accounts.escrow.key().as_ref(),
        &[ctx.accounts.escrow_authority.bump],
    ];
    let signer_seeds: &[&[&[u8]]] = &[&seeds];

    let cpi_ctx = CpiContext::new_with_signer(token_program, cpi_accounts, signer_seeds);

    // Probably whole vault: escrow.amount_a or vault_ata_a.amount
    transfer_checked(cpi_ctx, escrow.amount_a, ctx.accounts.mint_a.decimals)?;

    Ok(())
}

fn pay_taker_to_maker(
    token_program: AccountInfo,
    ctx: &Context<TakeEscrow>,
    escrow: &Escrow,
) -> Result<()> {
    let cpi_accounts = TransferChecked {
        from: ctx.accounts.taker_ata_b.to_account_info(),
        mint: ctx.accounts.mint_b.to_account_info(),
        to: ctx.accounts.maker_ata_b.to_account_info(),
        authority: ctx.accounts.taker.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(token_program, cpi_accounts);

    // You’ll probably store `escrow.amount_b` in Escrow
    transfer_checked(cpi_ctx, escrow.amount_b, ctx.accounts.mint_b.decimals)?;

    Ok(())
}

#[account]
pub struct Escrow {
    pub maker: Pubkey,
    pub taker: Pubkey,

    pub mint_a: Pubkey,
    pub mint_b: Pubkey,

    pub amount_a: u64,
    pub amount_b: u64,

    pub trade_id: u64,
    pub bump: u8,
}

#[account]
pub struct EscrowAuthority {
    pub escrow: Pubkey, //which escrow this authority belong to
    pub bump: u8,
}

#[derive(Accounts)]
#[instruction(trade_id: u64)]
pub struct InitEscrow<'info> {
    #[account(
        init,
        seeds=[
            b"escrow",
            maker.key().as_ref(),
            taker.key().as_ref(),
            &trade_id.to_le_bytes(),
        ],
        bump,
        payer=maker,
        space=8+32+32+32+8+8+1, //disc+maker+taker+mint+amount+trade_id+bump
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        init,
        seeds=[
            b"escrow_authority",
            escrow.key().as_ref(),
        ],
        bump,
        payer=maker,
        space=8+32+1, // disc + escrow pubkey + bump
    )]
    pub escrow_authority: Account<'info, EscrowAuthority>,

    #[account(mut)]
    pub maker: Signer<'info>,

    ///CHECK: we'll just pass taker pubkey in for now
    pub taker: UncheckedAccount<'info>,

    ///CHECK: token mint; placeholder for now
    pub mint_a: UncheckedAccount<'info>,

    pub mint_b: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositToEscrow<'info> {
    // Escrow PDA: re-derive from seeds + stored bump
    #[account(
        mut,
        seeds = [
            b"escrow",
            maker.key().as_ref(),
            taker.key().as_ref(),
            &escrow.trade_id.to_le_bytes(),
        ],
        bump = escrow.bump,
    )]
    pub escrow: Account<'info, Escrow>,

    // EscrowAuthority PDA: derived from escrow key + stored bump, must have one escrow
    #[account(
        mut,
        seeds = [
            b"escrow_authority",
            escrow.key().as_ref(),
        ],
        bump = escrow_authority.bump,
        has_one = escrow @ ErrorCode::InvalidEscrow,
    )]
    pub escrow_authority: Account<'info, EscrowAuthority>,

    /// CHECK: placeholder for SPL token vault; later swap to TokenAccount
    #[account(mut)]
    pub vault: UncheckedAccount<'info>,

    #[account(mut)]
    pub maker: Signer<'info>,

    /// CHECK: taker just as pubkey; same as InitEscrow
    pub taker: UncheckedAccount<'info>,

    /// CHECK: mint as pubkey; same as InitEscrow
    pub mint_a: UncheckedAccount<'info>,
}

#[derive(Accounts)]
//the taker pays token b to the maker, receives token a from the vault, vault+escrow can be closed and rent refunded
pub struct TakeEscrow<'info> {
    // Signer / system accounts
    #[account(mut)]
    pub taker: Signer<'info>,

    #[account(mut)]
    pub maker: SystemAccount<'info>,

    // Escrow state PDA
    #[account(
        mut,
        seeds = [
            b"escrow",
            maker.key().as_ref(),
            taker.key().as_ref(),
            &escrow.trade_id.to_le_bytes(),
        ],
        bump = escrow.bump,
    )]
    pub escrow: Account<'info, Escrow>,

    // Escrow authority PDA that owns the vault
    #[account(
        mut,
        seeds = [
            b"escrow_authority",
            escrow.key().as_ref(),
        ],
        bump = escrow_authority.bump,
        has_one = escrow @ ErrorCode::InvalidEscrow,
    )]
    pub escrow_authority: Account<'info, EscrowAuthority>,

    // Mints
    pub mint_a: InterfaceAccount<'info, Mint>, // Token maker deposited
    pub mint_b: InterfaceAccount<'info, Mint>, // Token maker wants

    // Vault holding Token A, owned by escrow_authority
    #[account(
        mut,
        token::mint = mint_a,
        token::authority = escrow_authority,
        token::token_program = token_program,
    )]
    pub vault_ata_a: InterfaceAccount<'info, TokenAccount>,

    // Taker accounts
    #[account(
        mut,
        token::mint = mint_a,
        token::authority = taker,
        token::token_program = token_program,
    )]
    pub taker_ata_a: InterfaceAccount<'info, TokenAccount>, // receives Token A

    #[account(
        mut,
        token::mint = mint_b,
        token::authority = taker,
        token::token_program = token_program,
    )]
    pub taker_ata_b: InterfaceAccount<'info, TokenAccount>, // sends Token B

    // Maker receives Token B here
    #[account(
        mut,
        token::mint = mint_b,
        token::authority = maker,
        token::token_program = token_program,
    )]
    pub maker_ata_b: InterfaceAccount<'info, TokenAccount>,

    // Token program (interface)
    pub token_program: Interface<'info, TokenInterface>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid escrow for this authority")]
    InvalidEscrow,

    #[msg("Invalid vault authority")]
    InvalidVaultAuthority,

    #[msg("Invalid vault mint")]
    InvalidVaultMint,
}
