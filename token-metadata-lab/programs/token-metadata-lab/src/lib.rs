use anchor_lang::prelude::*;

declare_id!("4jzvPnduASiFC6LT7b27Sq588X87cN5VjStkpR4a86aN");

#[program]
pub mod token_metadata_lab {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
