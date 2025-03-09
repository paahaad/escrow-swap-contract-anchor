use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Offer {
    pub id: i64,
    pub maker: Pubkey,
    pub token_mint_a: Pubkey,
    pub tokne_mint_b: Pubkey,
    pub wannted_amount: u64,
    pub dump: u8,
}
