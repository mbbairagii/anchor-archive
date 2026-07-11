use anchor_lang::prelude::*;
use crate::state::CircleState;

pub fn handler(ctx: Context<UpdateMemberCount>, new_count: u64) -> Result<()> {
    let circle = &mut ctx.accounts.circle_state;
    circle.member_count = new_count;
    Ok(())
}

#[derive(Accounts)]
#[instruction(circle_id: u64)]
pub struct UpdateMemberCount<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [b"circle-state", owner.key().as_ref(), &circle_id.to_le_bytes()],
        bump,
        has_one = owner
    )]
    pub circle_state: Account<'info, CircleState>,
}