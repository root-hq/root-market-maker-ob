use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer};

use crate::constants::DEPOSIT_RECEIPT_SEED;
// use crate::errors::*;
use crate::state::{DepositReceipt, UnifiedVault};
use crate::utils::match_deposit_ratios;

pub fn deposit_funds(
    ctx: Context<DepositFunds>,
    base_token_amount: u64,
    quote_token_amount: u64,
) -> Result<()> {
    // let clock = Clock::get()?;
    // let current_timestamp = clock.unix_timestamp as u64;

    // require!(
    //     current_timestamp >= ctx.accounts.vault.downtime_start_timestamp
    //         && current_timestamp < ctx.accounts.vault.downtime_end_timestamp,
    //     RootError::DepositWithdrawDuringUptime
    // );

    *ctx.accounts.deposit_receipt = DepositReceipt {
        bump: *ctx.bumps.get("deposit_receipt").unwrap(),
        owner: ctx.accounts.owner.key(),
        vault: ctx.accounts.vault.key(),
        base_token_liquidity_shares: 0u64,
        quote_token_liquidity_shares: 0u64,
    };

    let (base_new_deposits, quote_new_deposits) = match match_deposit_ratios(
        ctx.accounts.base_token_vault_ac.amount,
        ctx.accounts.quote_token_vault_ac.amount,
        base_token_amount,
        quote_token_amount,
    ) {
        Ok((adjusted_base_deposits, adjusted_quote_deposits)) => {
            (adjusted_base_deposits, adjusted_quote_deposits)
        }
        Err(e) => {
            msg!("Something went wrong matching deposit ratios: {}", e);
            (0, 0)
        }
    };

    // Calculate shares for base_token
    if ctx.accounts.base_token_vault_ac.amount == 0
        || ctx.accounts.vault.base_token_total_liquidity_shares == 0
    {
        ctx.accounts.vault.base_token_total_liquidity_shares += base_new_deposits;
        ctx.accounts.deposit_receipt.base_token_liquidity_shares += base_new_deposits;
    } else {
        let fresh_base_liquidity_shares = (base_new_deposits as u128)
            .checked_mul(ctx.accounts.vault.base_token_total_liquidity_shares as u128)
            .unwrap()
            .checked_div(ctx.accounts.base_token_vault_ac.amount as u128)
            .unwrap() as u64;

        ctx.accounts.vault.base_token_total_liquidity_shares += fresh_base_liquidity_shares;
        ctx.accounts.deposit_receipt.base_token_liquidity_shares += fresh_base_liquidity_shares;
    }

    // Calculate shares for quote_token
    if ctx.accounts.quote_token_vault_ac.amount == 0
        || ctx.accounts.vault.quote_token_total_liquidity_shares == 0
    {
        ctx.accounts.vault.quote_token_total_liquidity_shares += quote_new_deposits;
        ctx.accounts.deposit_receipt.quote_token_liquidity_shares += quote_new_deposits;
    } else {
        let fresh_quote_liquidity_shares = (quote_new_deposits as u128)
            .checked_mul(ctx.accounts.vault.quote_token_total_liquidity_shares as u128)
            .unwrap()
            .checked_div(ctx.accounts.quote_token_vault_ac.amount as u128)
            .unwrap() as u64;

        ctx.accounts.vault.quote_token_total_liquidity_shares += fresh_quote_liquidity_shares;
        ctx.accounts.deposit_receipt.quote_token_liquidity_shares += fresh_quote_liquidity_shares;
    }

    if base_new_deposits > 0 {
        anchor_spl::token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.base_token_user_ac.to_account_info(),
                    to: ctx.accounts.base_token_vault_ac.to_account_info(),
                    authority: ctx.accounts.owner.to_account_info(),
                },
            ),
            base_new_deposits,
        )?;
    }

    if quote_new_deposits > 0 {
        anchor_spl::token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.quote_token_user_ac.to_account_info(),
                    to: ctx.accounts.quote_token_vault_ac.to_account_info(),
                    authority: ctx.accounts.owner.to_account_info(),
                },
            ),
            quote_new_deposits,
        )?;
    }

    Ok(())
}

#[derive(Accounts)]
pub struct DepositFunds<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    /// CHECK: No constraint needed
    pub market_identifier: UncheckedAccount<'info>,

    #[account(
        mut,
        has_one = market_identifier,
        has_one = base_token_vault_ac,
        has_one = quote_token_vault_ac,
    )]
    pub vault: Box<Account<'info, UnifiedVault>>,

    #[account(
        init,
        seeds = [
            DEPOSIT_RECEIPT_SEED.as_bytes(),
            owner.key().as_ref(),
            vault.key().as_ref()
        ],
        bump,
        space = DepositReceipt::LEN,
        payer = owner
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

    pub system_program: Program<'info, System>,
}
