use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer};

// use crate::errors::*;
use crate::{
    constants::{DEPOSIT_RECEIPT_SEED, TRADE_MANAGER_SEED},
    state::{DepositReceipt, UnifiedVault},
};

pub fn withdraw_all_funds(ctx: Context<WithdrawAllFunds>) -> Result<()> {
    // let clock = Clock::get()?;
    // let current_timestamp = clock.unix_timestamp as u64;

    // require!(
    //     current_timestamp >= ctx.accounts.vault.downtime_start_timestamp
    //         && current_timestamp < ctx.accounts.vault.downtime_end_timestamp,
    //     RootError::DepositWithdrawDuringUptime
    // );

    let vault = &mut ctx.accounts.vault;
    let vault_key = vault.key();

    // Get signer seeds
    let trade_manager_bump = *ctx.bumps.get("trade_manager").unwrap();

    let trade_manager_seeds = &[
        TRADE_MANAGER_SEED.as_bytes(),
        vault_key.as_ref(),
        &[trade_manager_bump],
    ];
    let trade_manager_signer_seeds = &[&trade_manager_seeds[..]];

    let base_token_balance_deduction = (ctx.accounts.deposit_receipt.base_token_liquidity_shares
        as u128)
        .checked_mul(ctx.accounts.base_token_vault_ac.amount as u128)
        .unwrap()
        .checked_div(ctx.accounts.vault.base_token_total_liquidity_shares as u128)
        .unwrap() as u64;

    let quote_token_balance_deduction = (ctx.accounts.deposit_receipt.quote_token_liquidity_shares
        as u128)
        .checked_mul(ctx.accounts.quote_token_vault_ac.amount as u128)
        .unwrap()
        .checked_div(ctx.accounts.vault.quote_token_total_liquidity_shares as u128)
        .unwrap() as u64;

    ctx.accounts.vault.base_token_total_liquidity_shares -=
        ctx.accounts.deposit_receipt.base_token_liquidity_shares;
    ctx.accounts.vault.quote_token_total_liquidity_shares -=
        ctx.accounts.deposit_receipt.quote_token_liquidity_shares;

    if base_token_balance_deduction > 0 {
        anchor_spl::token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.base_token_vault_ac.to_account_info(),
                    to: ctx.accounts.base_token_user_ac.to_account_info(),
                    authority: ctx.accounts.trade_manager.to_account_info(),
                },
                trade_manager_signer_seeds,
            ),
            base_token_balance_deduction,
        )?;
    }

    if quote_token_balance_deduction > 0 {
        anchor_spl::token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.quote_token_vault_ac.to_account_info(),
                    to: ctx.accounts.quote_token_user_ac.to_account_info(),
                    authority: ctx.accounts.trade_manager.to_account_info(),
                },
                trade_manager_signer_seeds,
            ),
            quote_token_balance_deduction,
        )?;
    }

    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawAllFunds<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    /// CHECK: No constraint needed
    pub market_identifier: UncheckedAccount<'info>,

    #[account(
        mut,
        has_one = market_identifier,
        has_one = base_token_vault_ac,
        has_one = quote_token_vault_ac,
        has_one = trade_manager
    )]
    pub vault: Box<Account<'info, UnifiedVault>>,

    #[account(
        seeds = [
            TRADE_MANAGER_SEED.as_bytes(),
            vault.key().as_ref(),
        ],
        bump,
    )]
    /// CHECK: No constraint needed
    pub trade_manager: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [
            DEPOSIT_RECEIPT_SEED.as_bytes(),
            owner.key().as_ref(),
            vault.key().as_ref()
        ],
        bump = deposit_receipt.bump,
        close = owner
    )]
    pub deposit_receipt: Account<'info, DepositReceipt>,

    #[account(mut)]
    pub base_token_user_ac: Account<'info, TokenAccount>,

    #[account(mut)]
    pub quote_token_user_ac: Account<'info, TokenAccount>,

    #[account(mut)]
    pub base_token_vault_ac: Account<'info, TokenAccount>,

    #[account(mut)]
    pub quote_token_vault_ac: Account<'info, TokenAccount>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,
}
