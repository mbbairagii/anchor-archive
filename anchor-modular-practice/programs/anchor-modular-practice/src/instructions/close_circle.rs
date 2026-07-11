use anchor_lang::prelude::*;
use crate::state::CircleState;

pub fn handler(_ctx: Context<CloseCircle>, _circle_id: u64) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
#[instruction(circle_id: u64)]
pub struct CloseCircle<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [b"circle-state", owner.key().as_ref(), &circle_id.to_le_bytes()],
        bump,
        has_one = owner,
        close = owner
    )]
    pub circle_state: Account<'info, CircleState>,
}