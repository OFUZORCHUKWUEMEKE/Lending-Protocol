use anchor_lang::prelude::*;

#[error_code]
pub emum ErrorCode{
    #[msg("Insufficient funds")]
    InsufficientFunds
}