use anchor_lang::prelude::*;
use anchor_spl::token::{self, CloseAccount, Mint, SetAuthority, TokenAccount, Transfer};
use crate::state::{ReceiverDetails, ReceiverLamportDetails};

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

pub fn transfer_to_lamport_receiver(ctx: Context<TransferLamports>, avail_amount: u64) -> Result<()> {
    let sender = &mut ctx.accounts.sender.key.clone();
    let receiver = &mut ctx.accounts.receiver.receiver_pubkey.clone();
    let amount = ctx.accounts.receiver.amount.clone();
    if amount > avail_amount {
        panic!()
    }
    /*let ix = anchor_lang::solana_program::system_instruction::transfer(
        sender,
        receiver,
        amount,
    );
    msg!(&sender.to_string());
    msg!(&receiver.to_string());
    anchor_lang::solana_program::program::invoke(
        &ix,
        &[
            ctx.accounts.sender.to_account_info()
        ]
    );
    Ok(())*/
    **ctx.accounts.sender.try_borrow_mut_lamports()? -= amount;
    **ctx.accounts.getter.try_borrow_mut_lamports()? += amount;
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

#[derive(Accounts)]
pub struct TransferLamports<'info> {
    #[account(mut)]
    sender: Signer<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    getter: AccountInfo<'info>,
    #[account(mut)]
    receiver: Account<'info, ReceiverLamportDetails>,
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

