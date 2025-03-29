use anchor_lang::prelude::*;
use anchor_lang::solana_program::lamports;
use crate::constants::{
    CONDITION_SEED,
    LOCKUP_SEED
};
use crate::condition::Condition;
use anchor_lang::system_program::{
    transfer,
    Transfer
};

#[derive(Accounts)]
#[instruction(
    payment_index: u64,
)]
pub struct FinalizePayment<'info> {
    #[account(
        mut
    )]
    pub payer: Signer<'info>,

    /// CHECK: Up to payer
    #[account(
        mut
    )]
    pub recipient: AccountInfo<'info>,

    #[account(
        mut,
        close = payer,
        seeds = [
            CONDITION_SEED.as_bytes(),
            &payment_index.to_le_bytes()
        ],
        bump,
        has_one = recipient,
        // constraint = condition.recipient == recipient.key()
        has_one = payer,
        // constraint = condition.payer == payer.key(),
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
    ctx: Context<FinalizePayment>,
    payment_index: u64,
) -> Result<()> {

    let lockup = &ctx.accounts.lockup;
    let recipient = &ctx.accounts.recipient;
    let system_program = &ctx.accounts.system_program;

    let lamports = lockup.lamports();

    let signer_seeds = &[
        LOCKUP_SEED.as_bytes(),
        &payment_index.to_le_bytes(),
        &[ctx.bumps.lockup]
    ];

    transfer(
        CpiContext::new_with_signer(
            system_program.to_account_info(), 
            Transfer { 
                from: lockup.to_account_info(), 
                to: recipient.to_account_info() 
            },
            &[signer_seeds]
        ), 
        lamports
    )?;

    Ok(())
}