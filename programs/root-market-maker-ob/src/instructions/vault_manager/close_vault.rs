use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;

use crate::{errors::RootError, state::UnifiedVault};

use crate::constants::TRADE_MANAGER_SEED;

pub fn close_vault(ctx: Context<CloseVault>) -> Result<()> {
    let vault = &ctx.accounts.vault;

    // TODO - Replace vault.token_balance with token_vault_ac.amount
    require!(
        vault.base_token_balance == 0 && vault.base_token_total_liquidity_shares == 0,
        RootError::VaultFundsNotEmpty
    );
    require!(
        vault.quote_token_balance == 0 && vault.quote_token_total_liquidity_shares == 0,
        RootError::VaultFundsNotEmpty
    );

    let trade_manager_balance = ctx.accounts.trade_manager.lamports();

    // Get signer seeds
    let trade_manager_bump = *ctx.bumps.get("trade_manager").unwrap();

    let vault_key = ctx.accounts.vault.key();

    let trade_manager_seeds = &[
        TRADE_MANAGER_SEED.as_bytes(),
        vault_key.as_ref(),
        &[trade_manager_bump],
    ];
    let trade_manager_signer_seeds = &[&trade_manager_seeds[..]];

    // Transfer some SOL to trade_manager for seat initialization later
    let transfer_ix = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.trade_manager.key(),
        &ctx.accounts.owner.key(),
        trade_manager_balance,
    );
    invoke_signed(
        &transfer_ix,
        &[
            ctx.accounts.owner.to_account_info(),
            ctx.accounts.trade_manager.to_account_info(),
        ],
        trade_manager_signer_seeds,
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct CloseVault<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        has_one = owner,
        close = owner
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

    pub system_program: Program<'info, System>,
}
