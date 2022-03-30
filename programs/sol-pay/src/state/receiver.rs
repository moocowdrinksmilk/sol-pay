use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Transfer};

#[derive(Accounts)]
pub struct Receiver<'info> {
    #[account(init, payer = user, space = 8 + 40)]
    pub receiver: Account<'info, ReceiverDetails>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub receiver_token_account: Account<'info, TokenAccount>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account()]
pub struct ReceiverDetails {
    pub amount: u64, 
    pub token_account: Pubkey
}
impl<'info> Receiver <'info> {
    fn into_receiver_token_account_context(&self, sender_account: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: sender_account.clone(),
            to:self.receiver_token_account.to_account_info().clone(),
            authority: sender_account.clone()
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }
}
