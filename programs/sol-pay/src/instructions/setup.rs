use crate::state::receiver::*;
use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

pub fn setup_receiver(ctx: Context<InitializeReceiver>, amount: u64) -> Result<()> {
    let receiver = &mut ctx.accounts.receiver;
    receiver.amount = amount;
    receiver.token_account = ctx.accounts.receiver_token_account.key();
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeReceiver<'info> {
    #[account(init, payer = user, space = 8 + 72)]
    pub receiver: Account<'info, ReceiverDetails>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub receiver_token_account: Account<'info, TokenAccount>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}