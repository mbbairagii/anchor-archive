use anchor_lang::prelude::*;
use crate::state::MyState;

pub fn handler(ctx: Context<Initialize>, initial_value: u64) -> Result<()> {
    let state = &mut ctx.accounts.my_state;
    state.owner = ctx.accounts.payer.key();
    state.value = initial_value;
    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init, 
        payer = payer,
        space=8+MyState::LEN,
        seeds = [b"my-state", payer.key().as_ref()],
        bump
    )]

    pub my_state:Account<'info, MyState>,
    pub system_program: Program<'info, System>,
}