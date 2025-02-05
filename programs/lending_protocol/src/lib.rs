use anchor_lang::prelude::*;
use instructions::*;
mod state;
mod instructions;
mod error;
use error::*;
declare_id!("4ENquh7jVZk66cAeek3EUA7qPZoUWnnckGr95sBvqvnJ");

#[program]
pub mod lending_protocol {
    use super::*;

    pub fn init_bank(
        ctx: Context<InitBank>,
        liquidation_threshold: u64,
        max_ltv: u64
    ) -> Result<()> {
        process_initBank(ctx, liquidation_threshold, max_ltv)
    }

    pub fn init_user(ctx: Context<InitUser>, usdc_address: Pubkey) -> Result<()> {
        process_initUser(ctx, usdc_address)
    }

    // pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    //     process_deposit(ctx, amount)
    // }

    pub fn withdraw(ctx:Context<Withdraw>,amount:u64)->Result<()>{
        process_withdraw(ctx,amount)
    }
}
