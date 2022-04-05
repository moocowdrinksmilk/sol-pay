use anchor_lang::prelude::*;
use instructions::*;

pub mod state;
pub mod instructions;
declare_id!("HS59oAVB73bP2gnrU3eeaUfGkpQwoBLsByMH7AAwsCwe");

#[program]
pub mod sol_pay {
    use super::*;

    pub fn initialize(ctx: Context<InitializeReceiver>, amount: u64) -> Result<()> {
        instructions::setup_receiver(ctx, amount)
    }
    
    pub fn initialize_lamports(ctx: Context<LamportReceiver>, amount: u64) -> Result<()> {
        instructions::setup_lamport_receiver(ctx, amount)
    }
    
    pub fn transfer(ctx: Context<TransferToken>, avail_amount: u64) -> Result<()> {
        instructions::transfer_to_receiver(ctx, avail_amount)
    }
    
    pub fn transfer_lamports(ctx: Context<TransferLamports>, avail_amount: u64) -> Result<()> {
        instructions::transfer_to_lamport_receiver(ctx, avail_amount)
    }
}
