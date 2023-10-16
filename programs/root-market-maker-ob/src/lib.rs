use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

use crate::utils::OrderParams;
use instructions::*;

declare_id!("Bf7X9xMm4MNDpg8CjRvRwuyAabZPdQtDFH2kLPTPNuur");

#[program]
pub mod root_market_maker {

    use crate::utils::OrderParams;

    use super::*;

    pub fn initialize_vault(
        ctx: Context<InitializeVault>,
        cycle_duration_in_seconds: u64,
        downtime_in_seconds: u64,
    ) -> Result<()> {
        instructions::initialize_vault(ctx, cycle_duration_in_seconds, downtime_in_seconds)
    }

    pub fn close_vault(ctx: Context<CloseVault>) -> Result<()> {
        instructions::close_vault(ctx)
    }

    pub fn deposit_funds(
        ctx: Context<DepositFunds>,
        base_token_amount: u64,
        quote_token_amount: u64,
    ) -> Result<()> {
        instructions::deposit_funds(ctx, base_token_amount, quote_token_amount)
    }

    pub fn withdraw_all_funds(ctx: Context<WithdrawAllFunds>) -> Result<()> {
        instructions::withdraw_all_funds(ctx)
    }

    pub fn refresh_quotes_on_openbook_v2(
        ctx: Context<RefreshQuotesOnOpenbookV2>,
        bid_order_param: OrderParams,
        ask_order_param: OrderParams,
        cancel_only_mode: bool,
    ) -> Result<()> {
        instructions::refresh_quotes_on_openbook_v2(
            ctx,
            bid_order_param,
            ask_order_param,
            cancel_only_mode
        )
    }
}
