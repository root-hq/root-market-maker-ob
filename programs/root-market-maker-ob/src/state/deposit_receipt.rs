use anchor_lang::prelude::*;

#[derive(Debug, Default)]
#[account]
pub struct DepositReceipt {
    pub bump: u8,
    pub owner: Pubkey,
    pub vault: Pubkey,
    pub base_token_liquidity_shares: u64,
    pub quote_token_liquidity_shares: u64,
}

impl DepositReceipt {
    pub const LEN: usize = 8 + (2 * 36) + (2 * 8);
}
