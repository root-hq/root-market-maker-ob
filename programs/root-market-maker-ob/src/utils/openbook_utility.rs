use anchor_lang::prelude::*;

#[derive(Eq, PartialEq, Copy, Clone, Debug, AnchorSerialize, AnchorDeserialize)]
#[repr(u8)]
pub enum SelfTradeBehavior {
    DecrementTake = 0,
    CancelProvide = 1,
    AbortTransaction = 2,
}

impl From<SelfTradeBehavior> for openbook_v2::state::SelfTradeBehavior {
    fn from(behavior: SelfTradeBehavior) -> Self {
        match behavior {
            SelfTradeBehavior::DecrementTake => Self::DecrementTake,
            SelfTradeBehavior::CancelProvide => Self::CancelProvide,
            SelfTradeBehavior::AbortTransaction => Self::AbortTransaction,
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, AnchorSerialize, AnchorDeserialize)]
#[repr(u8)]
pub enum PlaceOrderType {
    Limit = 0,
    ImmediateOrCancel = 1,
    PostOnly = 2,
    Market = 3,
    PostOnlySlide = 4,
}

impl From<PlaceOrderType> for openbook_v2::state::PlaceOrderType {
    fn from(order_type: PlaceOrderType) -> Self {
        match order_type {
            PlaceOrderType::Limit => Self::Limit,
            PlaceOrderType::ImmediateOrCancel => Self::ImmediateOrCancel,
            PlaceOrderType::PostOnly => Self::PostOnly,
            PlaceOrderType::Market => Self::Market,
            PlaceOrderType::PostOnlySlide => Self::PostOnlySlide,
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, AnchorSerialize, AnchorDeserialize)]
#[repr(u8)]
pub enum Side {
    Bid = 0,
    Ask = 1,
}

impl From<Side> for openbook_v2::state::Side {
    fn from(side: Side) -> Self {
        match side {
            Side::Bid => Self::Bid,
            Side::Ask => Self::Ask,
        }
    }
}