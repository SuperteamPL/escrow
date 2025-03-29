pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("2FqJcVjBU9dt4PvWSnXD7KJhBNEoJs5T5j9y8SZKXFMV");

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize_payment(
        ctx: Context<InitializePayment>,
        payment_index: u64,
        condition_content: String,
        amount: u64,
    ) -> Result<()> {
        initialize_payment::handler(
            ctx,
            payment_index,
            condition_content,
            amount
        )
    }

    pub fn finalize_payment(
        ctx: Context<FinalizePayment>,
        payment_index: u64,
    ) -> Result<()> {
        finalize_payment::handler(ctx, payment_index)
    }
}
