use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Transfer};
use crate::state::ReceiverDetails;


#[derive(Accounts)]
pub struct TransferToken<'info> {
    #[account(mut)]
    sender: Signer<'info>,
    #[account(mut)]
    pub(crate) receiver: Account<'info, ReceiverDetails>,
    #[account(mut)]
    pub(crate) receiver_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub(crate) sender_token_account: Account<'info, TokenAccount>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    token_program: AccountInfo<'info>
}
