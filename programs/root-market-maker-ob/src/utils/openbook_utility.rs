use anchor_lang::prelude::*;

#[derive(Debug, AnchorDeserialize, AnchorSerialize, Clone, Copy)]
pub struct OrderParams {
    pub price_in_ticks: u64,
    pub size_in_base_lots: u64,
    pub strictly_post_only: bool,
    pub is_bid: bool,
}