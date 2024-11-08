use anchor_lang::prelude::*;

#[error_code]
pub emum ErrorCode{
    #[msg("Insufficient funds")]
    InsufficientFunds,
    #[msg("Requested exceeds Borrowable amount")]
    OverBorrowableAmount,
    #[msg("Attempting to repay more than borrowed.")]
    OverRepay,
    #[msg("Attempting to borrow more than allowed.")]
    OverBorrowableAmount,
    #[msg("User is not undercollateralized.")]
    NotUndercollateralized
}