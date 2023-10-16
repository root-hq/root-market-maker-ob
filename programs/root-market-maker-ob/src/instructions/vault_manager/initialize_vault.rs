use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::state::UnifiedVault;

use crate::constants::{
    BASE_TOKEN_VAULT_SEED, QUOTE_TOKEN_VAULT_SEED, SEAT_INITIALIZATION_PREMIUM, TRADE_MANAGER_SEED,
    UNIFIED_VAULT_SEED,
};

pub fn initialize_vault(
    ctx: Context<InitializeVault>,
    cycle_duration_in_seconds: u64,
    downtime_in_seconds: u64,
) -> Result<()> {
    let clock = Clock::get()?;
    let current_timestamp = clock.unix_timestamp as u64;

    **ctx.accounts.vault = UnifiedVault {
        bump: *ctx.bumps.get("vault").unwrap(),
        owner: ctx.accounts.owner.key(),
        market_identifier: ctx.accounts.market_identifier.key(),
        vault_identifier: ctx.accounts.vault_identifier.key(),
        trade_manager: ctx.accounts.trade_manager.key(),

        base_token_mint: ctx.accounts.base_token_mint.key(),
        quote_token_mint: ctx.accounts.quote_token_mint.key(),

        base_token_vault_ac: ctx.accounts.base_token_vault_ac.key(),
        quote_token_vault_ac: ctx.accounts.quote_token_vault_ac.key(),

        base_token_balance: 0u64,
        quote_token_balance: 0u64,

        base_token_total_liquidity_shares: 0u64,
        quote_token_total_liquidity_shares: 0u64,

        downtime_start_timestamp: current_timestamp,
        downtime_end_timestamp: current_timestamp + downtime_in_seconds,
        cycle_duration_in_seconds,
        downtime_in_seconds,

        last_update_slot: clock.slot,
    };

    // Transfer some SOL to trade_manager for seat initialization later
    let transfer_ix = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.owner.key(),
        &ctx.accounts.trade_manager.key(),
        SEAT_INITIALIZATION_PREMIUM,
    );
    invoke(
        &transfer_ix,
        &[
            ctx.accounts.owner.to_account_info(),
            ctx.accounts.trade_manager.to_account_info(),
        ],
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    /// CHECK: No constraint needed
    pub market_identifier: UncheckedAccount<'info>,

    /// CHECK: No constraint needed
    pub vault_identifier: UncheckedAccount<'info>,

    #[account(
        init,
        space = UnifiedVault::LEN,
        seeds = [
            UNIFIED_VAULT_SEED.as_bytes(),
            owner.key().as_ref(),
            market_identifier.key().as_ref(),
            vault_identifier.key().as_ref()
        ],
        bump,
        payer = owner
    )]
    pub vault: Box<Account<'info, UnifiedVault>>,

    #[account(
        mut,
        seeds = [
            TRADE_MANAGER_SEED.as_bytes(),
            vault.key().as_ref()
        ],
        bump,
    )]
    /// CHECK: NO constraint needed
    pub trade_manager: UncheckedAccount<'info>,

    pub base_token_mint: Account<'info, Mint>,

    pub quote_token_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = owner,
        seeds = [BASE_TOKEN_VAULT_SEED.as_bytes(), vault.key().as_ref()],
        bump,
        token::mint = base_token_mint,
        token::authority = trade_manager
    )]
    pub base_token_vault_ac: Box<Account<'info, TokenAccount>>,

    #[account(
        init,
        payer = owner,
        seeds = [QUOTE_TOKEN_VAULT_SEED.as_bytes(), vault.key().as_ref()],
        bump,
        token::mint = quote_token_mint,
        token::authority = trade_manager
    )]
    pub quote_token_vault_ac: Box<Account<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,
}
