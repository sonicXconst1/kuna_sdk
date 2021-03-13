use crate::base;
use crate::coin;
use crate::models;
use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct CanceledOrder {
    pub symbol: coin::Coins,
    pub side: base::Side,
    pub target: base::Target,
    pub price: f64,
    pub original_amount: f64,
    pub remaining_amount: f64,
}

impl TryFrom<models::CanceledOrderResponse> for CanceledOrder {
    type Error = &'static str;

    fn try_from(order: models::CanceledOrderResponse) -> Result<CanceledOrder, Self::Error> {
        Ok(CanceledOrder {
            symbol: coin::Coins::try_from(order.symbol.as_ref())?,
            side: base::Side::try_from(order.side.as_ref())?,
            target: base::Target::try_from(order.order_type.as_ref())?,
            price: f64::from_str(&order.price).expect("Invalid Number"),
            original_amount: f64::from_str(&order.original_amount).expect("Invalid number"),
            remaining_amount: f64::from_str(&order.remaining_amount).expect("Invalid number"),
        })
    }
}

#[derive(Clone, Debug)]
pub enum OrderStatus {
    Executed,
}
