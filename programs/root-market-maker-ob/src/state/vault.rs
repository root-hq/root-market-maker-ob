use anchor_lang::prelude::*;

#[derive(Debug, Default)]
#[account]
pub struct UnifiedVault {
    pub bump: u8,
    pub owner: Pubkey,
    pub vault_identifier: Pubkey,
    pub market_identifier: Pubkey,
    pub trade_manager: Pubkey,

    pub base_token_mint: Pubkey,
    pub quote_token_mint: Pubkey,

    pub base_token_vault_ac: Pubkey,
    pub quote_token_vault_ac: Pubkey,

    pub base_token_balance: u64,
    pub quote_token_balance: u64,

    pub base_token_total_liquidity_shares: u64,
    pub quote_token_total_liquidity_shares: u64,

    pub downtime_start_timestamp: u64,
    pub downtime_end_timestamp: u64,
    pub cycle_duration_in_seconds: u64,
    pub downtime_in_seconds: u64,

    pub last_update_slot: u64,
}

impl UnifiedVault {
    pub const LEN: usize = 8 + (1 * 1) + (8 * 36) + (7 * 8);
}
