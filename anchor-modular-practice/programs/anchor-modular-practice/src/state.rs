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

#[account]
pub struct  CircleState{
    pub owner: Pubkey,
    pub circle_id : u64,
    pub member_count: u64,
    pub name: String,
}

impl CircleState {
    pub const MAX_NAME_LEN: usize =50;
    pub const LEN: usize = 32+8+8+4+Self::MAX_NAME_LEN;
}