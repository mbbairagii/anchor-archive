//This file is only for on-chain account structs and their size constants.

use anchor_lang::prelude::*;

#[account]
pub struct MyState{
    pub owner: Pubkey,
    pub value: u64,
}

impl MyState {
    pub const LEN: usize = 32+8;
}