//lib.rs should be thin: just declare modules, import what you need, 
//and expose the instruction entrypoints through #[program]

#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

declare_id!("2Aavozw3KGwy2PAu2snnJd7cNpbempvAAzx8CBjepirF");

//why lib.rs needs these lines
//lib.rs is the root of your crate/module tree. So if you want Rust to know 
//that state.rs, errors.rs, and instructions/ are part of the project, you 
//declare them there:
pub mod state; //There is a module called state, and its code is in state.rs. Also, make that module publicly accessible.
pub mod errors;
pub mod instructions;

pub use instructions::*;
pub use state::*;

#[program]
pub mod anchor_modular_practice {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, initial_value: u64) -> Result<()> {
        instructions::initialize::handler(ctx, initial_value)
    }

    pub fn update_value(ctx: Context<UpdateValue>, new_value: u64) -> Result<()> {
        instructions::update_value::handler(ctx, new_value)
    }

    pub fn create_circle(ctx: Context<CreateCircle>, circle_id: u64, name: String) -> Result<()> {
        instructions::create_circle::handler(ctx, circle_id, name)
    }
}

