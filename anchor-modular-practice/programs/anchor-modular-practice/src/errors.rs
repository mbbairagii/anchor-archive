//one custom error.

use anchor_lang::prelude::*;

#[error_code]
pub enum MyError {
    #[msg("Only the owner can update this value")]
    Unauthorized,
}