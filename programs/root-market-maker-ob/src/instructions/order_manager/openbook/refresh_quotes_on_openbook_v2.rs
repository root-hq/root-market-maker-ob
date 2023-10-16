use anchor_lang::prelude::*;
use anchor_spl::token::Token;
use openbook_v2::{ state::{BookSide, Market}, program::OpenbookV2};

use openbook_v2::PlaceOrderArgs;

use crate::constants::TRADE_MANAGER_SEED;
use crate::state::UnifiedVault;

pub fn refresh_quotes_on_openbook_v2(
    ctx: Context<RefreshQuotesOnOpenbookV2>,
    bid_order_args: PlaceOrderArgs,
    ask_order_args: PlaceOrderArgs,
    cancel_only_mode: bool
) -> Result<()> {
    // Clock params
    let clock = Clock::get()?;
    let current_timestamp = clock.unix_timestamp as u64;
    ctx.accounts.vault.last_update_slot = clock.slot;

    msg!("Current timestamp: {}", current_timestamp);
    msg!("downtime_start_timestamp: {}", ctx.accounts.vault.downtime_start_timestamp);
    msg!("downtime_end_timestamp: {}", ctx.accounts.vault.downtime_end_timestamp);

    if current_timestamp > ctx.accounts.vault.downtime_end_timestamp {
        msg!("updating new downtime values");

        let next_downtime_start = ctx.accounts.vault.downtime_end_timestamp
            + ctx.accounts.vault.cycle_duration_in_seconds;
        let next_downtime_end = next_downtime_start + ctx.accounts.vault.downtime_in_seconds;

        msg!("new downtime_start: {}", next_downtime_start);
        msg!("new downtime_end: {}", next_downtime_end);
    
        ctx.accounts.vault.downtime_start_timestamp = next_downtime_start;
        ctx.accounts.vault.downtime_end_timestamp = next_downtime_end;
    }

    msg!("RefreshQuotesDuringDowntime condition: {}", !(current_timestamp >= ctx.accounts.vault.downtime_start_timestamp
    && current_timestamp < ctx.accounts.vault.downtime_end_timestamp));

    // Get signer seeds
    let trade_manager_bump = *ctx.bumps.get("trade_manager").unwrap();

    let vault_key = ctx.accounts.vault.key();

    let trade_manager_seeds = &[
        TRADE_MANAGER_SEED.as_bytes(),
        vault_key.as_ref(),
        &[trade_manager_bump],
    ];
    let trade_manager_signer_seeds = &[&trade_manager_seeds[..]];

    let cpi_program = ctx.accounts.openbook_program.to_account_info();

    // Cancel all old orders
    let cancel_cpi_accs = openbook_v2::cpi::accounts::CancelOrder {
        signer: ctx.accounts.trade_manager.to_account_info(),
        open_orders_account: ctx.accounts.open_orders_account.to_account_info(),
        market: ctx.accounts.market.to_account_info(),
        bids: ctx.accounts.bids.to_account_info(),
        asks: ctx.accounts.asks.to_account_info()
    };
    let cancel_cpi_ctx = CpiContext::new_with_signer(cpi_program.clone(), cancel_cpi_accs, trade_manager_signer_seeds);
    openbook_v2::cpi::cancel_all_orders(cancel_cpi_ctx, None, 2)?;

    if !cancel_only_mode {

        // Place bid order
        let cpi_accs = openbook_v2::cpi::accounts::PlaceOrder {
            signer: ctx.accounts.trade_manager.to_account_info(),
            open_orders_account: ctx.accounts.open_orders_account.to_account_info(),
            open_orders_admin: Some(ctx.accounts.trade_manager.to_account_info()),
            user_token_account: ctx.accounts.quote_token_vault_ac.to_account_info(),
            market: ctx.accounts.market.to_account_info(),
            bids: ctx.accounts.bids.to_account_info(),
            asks: ctx.accounts.asks.to_account_info(),
            event_heap: ctx.accounts.event_heap.to_account_info(),
            market_vault: ctx.accounts.quote_token_market_vault.to_account_info(),
            oracle_a: None,
            oracle_b: None,
            token_program: ctx.accounts.token_program.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accs, trade_manager_signer_seeds);
        openbook_v2::cpi::place_order(cpi_ctx, bid_order_args.into())?;

        // Place ask order
        let cpi_program = ctx.accounts.openbook_program.to_account_info();
        let cpi_accs = openbook_v2::cpi::accounts::PlaceOrder {
            signer: ctx.accounts.trade_manager.to_account_info(),
            open_orders_account: ctx.accounts.open_orders_account.to_account_info(),
            open_orders_admin: Some(ctx.accounts.trade_manager.to_account_info()),
            user_token_account: ctx.accounts.base_token_vault_ac.to_account_info(),
            market: ctx.accounts.market.to_account_info(),
            bids: ctx.accounts.bids.to_account_info(),
            asks: ctx.accounts.asks.to_account_info(),
            event_heap: ctx.accounts.event_heap.to_account_info(),
            market_vault: ctx.accounts.base_token_market_vault.to_account_info(),
            oracle_a: None,
            oracle_b: None,
            token_program: ctx.accounts.token_program.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accs, trade_manager_signer_seeds);
        openbook_v2::cpi::place_order(cpi_ctx, ask_order_args.into())?;
    }
    

    msg!("signer: {}", ctx.accounts.owner.key());
    msg!("Cancel only: {}", cancel_only_mode);
    
    Ok(())
}

#[derive(Accounts)]
pub struct RefreshQuotesOnOpenbookV2<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    pub signer: Signer<'info>,

    #[account(
        mut,
        has_one = owner,
        has_one = base_token_vault_ac,
        has_one = quote_token_vault_ac
    )]
    pub vault: Box<Account<'info, UnifiedVault>>,

    #[account(
        mut,
        seeds = [
            TRADE_MANAGER_SEED.as_bytes(),
            vault.key().as_ref(),
        ],
        bump,
    )]
    /// CHECK: No constraint needed
    pub trade_manager: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: Checked in CPI
    pub open_orders_account: UncheckedAccount<'info>,

    #[account(
        mut,
        address = vault.base_token_vault_ac.key()
    )]
    /// CHECK: Checked in CPI
    pub base_token_vault_ac: UncheckedAccount<'info>,

    #[account(
        mut,
        address = vault.quote_token_vault_ac.key()
    )]
    /// CHECK: Checked in CPI
    pub quote_token_vault_ac: UncheckedAccount<'info>,

    #[account(mut)]
    pub market: AccountLoader<'info, Market>,

    #[account(mut)]
    pub bids: AccountLoader<'info, BookSide>,

    #[account(mut)]
    pub asks: AccountLoader<'info, BookSide>,

    #[account(mut)]
    /// CHECK: Checked in CPI
    pub event_heap: UncheckedAccount<'info>,
    
    #[account(
        mut,
        constraint = base_token_market_vault.key() == market.load()?.market_base_vault.key()
    )]
    /// CHECK: Checked in CPI
    pub base_token_market_vault: UncheckedAccount<'info>,

    #[account(
        mut,
        constraint = quote_token_market_vault.key() == market.load()?.market_quote_vault.key()
    )]
    /// CHECK: Checked in CPI
    pub quote_token_market_vault: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,

    pub openbook_program: Program<'info, OpenbookV2>,

    pub system_program: Program<'info, System>,
}