use anchor_lang::prelude::*;
use instructions::*;
mod state;
mod error;
declare_id!("6eZg8GbWTQg8gLCnHCFuyPgmfU2LisqBi4rM5UEqZGai");

#[program]
pub mod lending_protocol {
    use super::*;

    pub fn init_bank(ctx:Context<InitBank>,liquidation_threshold:u64,max_ltv:u64)->Result<()>{
        process_init_bank(ctx,liquidation_threshold,max_ltv);
    }

    pub fn init_user(ctx:Context<InitUser>,usdc_address:Pubkey)->Result<()>{
        process_init_user(ctx,usdc_address);
    }

    pub fn deposit(ctx:Context<Deposit>,amount:u64)->Result<()>{
        process_deposit(ctx,amount)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
