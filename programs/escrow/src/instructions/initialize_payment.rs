use anchor_lang::prelude::*;
use crate::state::condition::Condition;
use crate::constants::{
    CONDITION_SEED,
    LOCKUP_SEED
};
use anchor_lang::system_program::{
    transfer,
    Transfer
};

#[derive(Accounts)]
#[instruction(
    payment_index: u64,
    condition_content: String,
)]
pub struct InitializePayment<'info> {
    #[account(
        mut
    )]
    pub payer: Signer<'info>,

    /// CHECK: Up to user
    #[account(mut)]
    pub recipient: AccountInfo<'info>,

    #[account(
        init,
        payer = payer,
        space = 8 + 2 * 32 + 4 + condition_content.len(),
        seeds = [
            CONDITION_SEED.as_bytes(),
            &payment_index.to_le_bytes()
        ],
        bump,
    )]
    pub condition: Account<'info, Condition>,

    /// CHECK: There is no data
    #[account(
        mut,
        seeds = [
            LOCKUP_SEED.as_bytes(),
            &payment_index.to_le_bytes()
        ],
        bump
    )]
    pub lockup: AccountInfo<'info>,

    #[account()]
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<InitializePayment>, 
    payment_index: u64,
    condition_content: String,
    amount: u64,
) -> Result<()> {
    let payer = &ctx.accounts.payer;
    let condition = &mut ctx.accounts.condition;
    let lockup = &ctx.accounts.lockup;
    let system_program = &ctx.accounts.system_program;
    let recipient = &ctx.accounts.recipient;

    condition.payer = payer.key();
    condition.content = condition_content;
    condition.recipient = recipient.key();

    msg!("payer: {:?}", payer.key());
    msg!("recipient: {:?}", recipient.key());
    msg!("lockup: {:?}", lockup.key());

    transfer(
        CpiContext::new(
            system_program.to_account_info(), 
            Transfer { 
                from: payer.to_account_info(), 
                to: lockup.to_account_info()
            }
        ), 
        amount
    )?;

    Ok(())
}
