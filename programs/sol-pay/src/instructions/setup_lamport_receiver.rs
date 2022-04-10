use anchor_lang::prelude::*;
use crate::state::ReceiverLamportDetails;

pub fn setup_lamport_receiver (ctx: Context<LamportReceiver>, amount: u64) ->Result<()> {
    let receiver = &mut ctx.accounts.receiver;
    receiver.receiver_pubkey = *ctx.accounts.user.key;
    receiver.amount = amount;
    Ok(())
}

#[derive(Accounts)]
pub struct LamportReceiver<'info> {
    #[account(init, payer = user, space = 8 + 72)]
    pub receiver: Account<'info, ReceiverLamportDetails>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}