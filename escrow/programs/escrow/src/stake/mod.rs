use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub seed: u64,
    pub maker: Pubkey,
    pub recieve: u64,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub bump: u8,
}
