use anchor_lang::prelude::*;
use crate::state::CircleState;

pub fn handler(ctx: Context<CreateCircle>, circle_id: u64, name: String) -> Result<()> {
    let circle = &mut ctx.accounts.circle_state;
    circle.owner = ctx.accounts.owner.key();
    circle.circle_id = circle_id;
    circle.member_count = 1;
    circle.name = name;
    Ok(())
}

#[derive(Accounts)]
#[instruction(circle_id: u64, name: String)]
pub struct CreateCircle<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = 8 + CircleState::LEN,
        seeds = [b"circle-state", owner.key().as_ref(), &circle_id.to_le_bytes()],
        bump
    )]
    pub circle_state: Account<'info, CircleState>,

    pub system_program: Program<'info, System>,
}