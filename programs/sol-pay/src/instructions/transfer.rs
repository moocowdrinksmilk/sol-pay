use anchor_lang::prelude::*;
use anchor_spl::token::{self, CloseAccount, Mint, SetAuthority, TokenAccount, Transfer};
use crate::state::ReceiverDetails;

pub fn transfer_to_receiver(ctx: Context<TransferToken>, avail_amount: u64) -> Result<()> {
    let transfer_amount = ctx.accounts.receiver.amount;
    if transfer_amount > avail_amount {
        // Send error
        panic!()
    }
    msg!("hi");
    token::transfer(
        ctx.accounts.into_tranfer_token_receiver_account(),
        transfer_amount
    );
    Ok(())
}

#[derive(Accounts)]
pub struct TransferToken<'info> {
    #[account(mut)]
    sender: Signer<'info>,
    #[account(mut)]
    receiver: Account<'info, ReceiverDetails>,
    #[account(mut)]
    receiver_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    sender_token_account: Account<'info, TokenAccount>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    token_program: AccountInfo<'info>
}

impl<'info> TransferToken<'info> {
    pub(crate) fn into_tranfer_token_receiver_account(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.sender_token_account.to_account_info().clone(),
            to: self.receiver_token_account.to_account_info().clone(),
            authority: self.sender.to_account_info().clone()
        };
        msg!("sender: {}", &self.sender_token_account.to_account_info().key.to_string());
        msg!("receiver: {}", &self.receiver_token_account.to_account_info().key.to_string());
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }
}
