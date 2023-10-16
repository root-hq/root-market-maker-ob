use anchor_lang::error_code;

#[error_code]
pub enum RootError {
    #[msg("Phoenix market header deserialization error")]
    PhoenixHeaderError,
    #[msg("Phoenix program id invalid")]
    InvalidPhoenixProgram,
    #[msg("Phoenix market deserialization error")]
    PhoenixMarketError,
    #[msg("Phoenix vault seat Retired")]
    PhoenixVaultSeatRetired,
    #[msg("Vault funds not empty")]
    VaultFundsNotEmpty,
    #[msg("Deposit/Withdraw only enabled during downtime period")]
    DepositWithdrawDuringUptime,
    #[msg("Cannot refresh quotes during downtime period")]
    RefreshQuotesDuringDowntime,
    #[msg("Deposit ratio check failed")]
    DepositRatioCheckFail,
}
