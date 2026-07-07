#![allow(unexpected_cfgs)] //silences noisy rust warnings about internal anchor/solana cfg flags, so ur screen isnt spammed
use anchor_lang::prelude::*; //imports anchor's core types/macros

declare_id!("HC2o6XV9tXYyQCHWfhWyzGaD1R3ht299YxeYfDGEYVD4"); //program's on-chain address

//#[program] + pub mod my_first_anchor_program: main module of my on-chain program 
//So the whole initialize instruction does:
//Create or use a my_state account and store two things in it: who called (owner) and the initial numeric value.”
#[program]
pub mod my_first_anchor_program {
    use super::*;

    //ctx: Context<Initialize> → a bundle of all accounts this instruction needs, initial_value: u64 → a number passed in by the caller.
    pub fn initialize(ctx: Context<Initialize>, initial_value: u64) -> Result<()> {
        let state = &mut ctx.accounts.my_state; //Grab a writable reference to the my_state account from the context.
        state.owner = ctx.accounts.payer.key(); //Set the owner field in that account’s data to the payer’s pubkey.
        state.value = initial_value; //Set the value field in that account’s data to the initial_value argument.
        Ok(())
    }

    pub fn update_value(ctx: Context<UpdateValue>, new_value: u64) -> Result<()> {
        let state = &mut ctx.accounts.my_state;

        //ensure only owner can update the value
        require!( //anchor's recommended way to check conditions and fail with an error
            ctx.accounts.payer.key() == state.owner,
            anchor_lang::error::ErrorCode::ConstraintOwner
        );
        state.value = new_value;
        Ok(())
    }

    //creates a new CircleState PDA for a given circle_id
    //seeds: circle-state + owner (we’ll add circle_id later once you’re comfortable)
    //sets initial fields: owner, circle_id, name, member_count
    pub fn create_circle(
        ctx: Context<CreateCircle>,
        circle_id: u64,
        name: String,
    ) -> Result<()> {
        let circle = &mut ctx.accounts.circle_state;
        circle.owner = ctx.accounts.owner.key();
        circle.circle_id = circle_id;
        circle.name = name;
        circle.member_count = 0;

        Ok(())
    }

    //reuses the same pda, lets the owner update member_count
    pub fn update_member_count(
        ctx: Context<UpdateMemberCount>,
        new_member_count: u64,
    ) -> Result<()> {
        let circle = &mut ctx.accounts.circle_state;

        //only the owner of the circle can update the member count
        require!(
            ctx.accounts.owner.key() == circle.owner,
            anchor_lang::error::ErrorCode::ConstraintOwner
        );

        circle.member_count = new_member_count;
        Ok(())
    }
}

//This struct describes all the accounts the initialize instruction needs, and what rules they must follow.
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>, //This is the user calling initialize. They must sign the transaction (Signer), and we’ll mark them mutable (mut) because we’ll charge them lamports to create the new account.

    #[account(
        init,
        payer = payer,
        space = 8 + MyState::LEN,
        seeds = [b"my-state", payer.key().as_ref()],
        bump
    )]
    pub my_state: Account<'info, MyState>, //This is the account that will hold our custom MyState data.

    pub system_program: Program<'info, System>,
}

//define the UpdateValue accounts struct (right below Initialize)
#[derive(Accounts)]
pub struct UpdateValue<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    //use same pda, we dont init this time , we expect it to already exist 
    #[account(
        mut, //this acc will be modified
        seeds = [b"my-state", payer.key().as_ref()], //this enforces that the same pda is derived
        bump
    )]
    pub my_state: Account<'info, MyState>,
}

//accounts for create_circle
#[derive(Accounts)]
pub struct CreateCircle<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = 8 + CircleState::LEN,
        seeds = [b"circle-state", owner.key().as_ref()],
        bump
    )]
    pub circle_state: Account<'info, CircleState>,

    pub system_program: Program<'info, System>,
}

//accounts for update_member_count
#[derive(Accounts)]
pub struct UpdateMemberCount<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [b"circle-state", owner.key().as_ref()],
        bump
    )]
    pub circle_state: Account<'info, CircleState>,
}

//This is the structure we will store inside my_state account’s data.
#[account]
pub struct MyState {
    pub owner: Pubkey,
    pub value: u64,
}

//for circle state
#[account]
pub struct CircleState {
    pub owner: Pubkey,
    pub circle_id: u64,
    pub name: String,
    pub member_count: u64,
}

impl MyState {
    pub const LEN: usize = 32 + 8; // Anchor uses LEN to know how much space to allocate when creating the account.
}

impl CircleState {
    // 32 bytes for Pubkey, 8 for circle_id, 4 for string length prefix, 32 for max name length, 8 for member_count
    pub const LEN: usize = 32 + 8 + 4 + 32 + 8;
}