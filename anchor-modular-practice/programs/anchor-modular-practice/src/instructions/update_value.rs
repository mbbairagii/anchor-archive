use anchor_lang::prelude::*;
use crate::state::MyState;
use crate::errors::MyError;

pub fn handler(ctx: Context<UpdateValue>, new_value: u64) -> Result<()> {
    let state = &mut ctx.accounts.my_state;

    require!(
        ctx.accounts.payer.key() == state.owner,
        MyError::Unauthorized
    );

    state.value = new_value;
    Ok(())
}

#[derive(Accounts)]
pub struct UpdateValue<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"my-state", payer.key().as_ref()],
        bump
    )]
    pub my_state: Account<'info, MyState>,
}