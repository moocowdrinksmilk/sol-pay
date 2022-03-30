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

impl<'info> TransferToken<'info> {
    pub(crate) fn into_tranfer_token_receiver_account(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.sender.to_account_info(),
            to: self.receiver_token_account.to_account_info(),
            authority: self.sender.to_account_info()
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }
}