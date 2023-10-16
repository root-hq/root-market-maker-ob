use anchor_lang::prelude::*;

use crate::utils::OrderParams;

pub fn refresh_quotes_on_openbook_v2(
    ctx: Context<RefreshQuotesOnOpenbookV2>,
    bid_order_param: OrderParams,
    ask_order_param: OrderParams,
    cancel_only_mode: bool
) -> Result<()> {

    msg!("signer: {}", ctx.accounts.owner.key());
    msg!("Placing bids: {}", bid_order_param.price_in_ticks);
    msg!("Placing asks: {}", ask_order_param.price_in_ticks);
    msg!("Cancel only: {}", cancel_only_mode);
    
    Ok(())
}

#[derive(Accounts)]
pub struct RefreshQuotesOnOpenbookV2<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    // #[account(
    //     mut,
    //     seeds = [b"OpenOrdersIndexer".as_ref(), owner.key().as_ref()],
    //     bump = open_orders_indexer.bump,
    //     realloc = OpenOrdersIndexer::space(open_orders_indexer.addresses.len()+1),
    //     realloc::payer = payer,
    //     realloc::zero = false,
    //     constraint = open_orders_indexer.addresses.len() < 256,
    // )]
    // pub open_orders_indexer: Account<'info, OpenOrdersIndexer>,
    // #[account(
    //     init,
    //     seeds = [b"OpenOrders".as_ref(), owner.key().as_ref(), market.key().as_ref(), &(open_orders_indexer.created_counter + 1).to_le_bytes()],
    //     bump,
    //     payer = payer,
    //     space = OpenOrdersAccount::space(),
    // )]
    // pub open_orders_account: AccountLoader<'info, OpenOrdersAccount>,
    // pub market: AccountLoader<'info, Market>,
    pub system_program: Program<'info, System>,
}