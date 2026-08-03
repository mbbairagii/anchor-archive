#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

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
        amount: u64,
    ) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;
        escrow.maker = ctx.accounts.maker.key();
        escrow.taker = ctx.accounts.taker.key();
        escrow.mint = ctx.accounts.mint.key();
        escrow.amount = amount;
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
            ctx.accounts.mint.key() == ctx.accounts.escrow.mint,
            ErrorCode::InvalidVaultMint
        );

        // You can’t check vault.owner here until vault is a TokenAccount,
        // but conceptually:
        // require!(vault.owner == escrow_authority.key(), ErrorCode::InvalidVaultAuthority);

        Ok(())
    }
}

#[account]
pub struct Escrow{
    pub maker: Pubkey,
    pub taker: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
    pub trade_id: u64,
    pub bump: u8,
}

#[account]
pub struct EscrowAuthority{
    pub escrow: Pubkey, //which escrow this authority belong to
    pub bump: u8
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
    pub escrow:Account<'info, Escrow>,

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
    pub mint: UncheckedAccount<'info>,

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
    pub mint: UncheckedAccount<'info>,
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
