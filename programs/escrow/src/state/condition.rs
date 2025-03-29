use anchor_lang::prelude::*;

#[account]
pub struct Condition {
    pub payer: Pubkey,
    pub recipient: Pubkey,
    pub content: String, // "Paczka została wysłana"
}