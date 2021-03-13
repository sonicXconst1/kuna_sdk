use crate::coin::Coins;
use crate::base::Target;
use crate::base::Side;

pub type Currencies = Vec<CurrencyEntries>;
pub type CurrencyEntries = (String, String, f64, Option<String>, f64);

#[derive(Debug, Clone)]
pub struct Currency {
    pub coin: crate::coin::Coin,
    pub full: f64,
    pub available: f64,
}

impl Currency {
    pub const COIN: usize = 1;
    pub const FULL: usize = 2;
    pub const AVAILABLE: usize = 4;

    pub fn from(currency: &CurrencyEntries) -> Option<Self> {
        let available = currency.4;
        let full = currency.2;
        Some(Currency {
            coin: crate::coin::Coin::from(currency.1.as_ref()),
            full,
            available,
        })
    }
}

impl From<&Currency> for CurrencyEntries {
    fn from(currency: &Currency) -> CurrencyEntries {
        (
            "exchange".to_owned(),
            currency.coin.to_string(),
            currency.full,
            None,
            currency.available,
        )
    }
}

pub type OrderBookEntry = (f64, f64, u32);
pub type OrderBookEntries = Vec<OrderBookEntry>;

#[derive(Clone, Debug)]
pub struct OrderBook {
    pub coins: crate::coin::Coins,
    pub bids: Vec<OrderBookItem>,
    pub asks: Vec<OrderBookItem>,
}

impl OrderBook {
    pub fn with(coins: crate::coin::Coins, raw_order_book: OrderBookEntries) -> OrderBook {
        let (bids, asks) =
            raw_order_book
                .into_iter()
                .fold((Vec::new(), Vec::new()), |mut acc, entry| {
                    if entry.1 > 0.0 {
                        acc.0.push(OrderBookItem::from(entry))
                    } else {
                        acc.1.push(OrderBookItem::from(entry))
                    }
                    acc
                });
        OrderBook { coins, bids, asks }
    }
}

#[derive(Clone, Debug)]
pub struct OrderBookItem {
    pub price: f64,
    pub amount: f64,
    pub orders_number: u32,
}

impl From<OrderBookEntry> for OrderBookItem {
    fn from(entry: OrderBookEntry) -> OrderBookItem {
        OrderBookItem {
            price: entry.0,
            amount: entry.1.abs(),
            orders_number: entry.2,
        }
    }
}

#[derive(serde::Serialize, Clone, Debug)]
pub struct CreateOrder {
    pub symbol: String,
    pub amount: f64,
    pub price: f64,
    #[serde(rename = "type")]
    pub order_type: String,
}

pub type CreateOrderResponseRaw = Vec<serde_json::Value>;

#[derive(Clone, Debug)]
pub struct CreateOrderResponse {
    pub id: i32,
    pub coins: crate::coin::Coins,
    pub side: crate::base::Side,
    pub target: crate::base::Target,
    pub initial_amount: f64,
    pub executed_amount: f64,
}

fn json_to_id(value: Option<&serde_json::Value>) -> Result<i32, CreateOrderError> {
    let value = match value {
        Some(value) => value,
        None => return Err(CreateOrderError::InvalidJson),
    };
    match value {
        serde_json::Value::Number(number) => match number.as_i64() {
            Some(id) => Ok(id as i32),
            None => Err(CreateOrderError::InvalidJson),
        },
        _ => Err(CreateOrderError::InvalidJson),
    }
}

fn json_to_amount(value: Option<&serde_json::Value>) -> Result<f64, CreateOrderError> {
    use std::str::FromStr;
    let value = match value {
        Some(value) => value,
        None => return Err(CreateOrderError::InvalidJson),
    };
    match value {
        serde_json::Value::String(as_string) => match f64::from_str(&as_string) {
            Ok(amount) => Ok(amount),
            Err(error) => Err(CreateOrderError::InvalidAmount(error)),
        },
        serde_json::Value::Number(number) => match number.as_f64() {
            Some(amount) => Ok(amount),
            None => Err(CreateOrderError::InvalidJson),
        },
        _ => Err(CreateOrderError::InvalidJson),
    }
}

fn json_to_coins(
    value: Option<&serde_json::Value>,
) -> Result<Coins, CreateOrderError> {
    use std::convert::TryFrom;
    let value = match value {
        Some(value) => value,
        None => return Err(CreateOrderError::InvalidJson),
    };
    match value {
        serde_json::Value::String(string) => match Coins::try_from(string.as_ref()) {
            Ok(coins) => Ok(coins),
            Err(_) => Err(CreateOrderError::InvalidJson),
        },
        _ => Err(CreateOrderError::InvalidJson),
    }
}

fn json_to_target(
    value: Option<&serde_json::Value>,
) -> Result<Target, CreateOrderError> {
    use std::convert::TryFrom;
    let value = match value {
        Some(value) => value,
        None => return Err(CreateOrderError::InvalidJson),
    };
    match value {
        serde_json::Value::String(string) => match Target::try_from(string.to_lowercase().as_ref()) {
            Ok(target) => Ok(target),
            Err(_) => Err(CreateOrderError::InvalidJson),
        },
        _ => Err(CreateOrderError::InvalidJson),
    }
}

impl std::convert::TryFrom<CreateOrderResponseRaw> for CreateOrderResponse {
    type Error = CreateOrderError;

    fn try_from(response: CreateOrderResponseRaw) -> Result<Self, Self::Error> {
        if response.len() < 17 {
            return Err(CreateOrderError::InvalidJson);
        };
        let initial_amount = json_to_amount(response.get(6))?;
        let executed_amount = json_to_amount(response.get(7))?;
        Ok(CreateOrderResponse {
            id: json_to_id(response.get(0))?,
            coins: json_to_coins(response.get(3))?,
            side: if initial_amount < 0.0 { Side::Sell } else { Side::Buy },
            target: json_to_target(response.get(8))?,
            initial_amount,
            executed_amount,
        })
    }
}

#[derive(Clone, Debug)]
pub enum CreateOrderError {
    InvalidCoins(&'static str),
    InvalidAmount(<f64 as std::str::FromStr>::Err),
    InvalidTarget(&'static str),
    InvalidJson,
}

impl CreateOrder {
    pub fn new(
        coins: crate::coin::Coins,
        side: crate::base::Side,
        target: crate::base::Target,
        amount: f64,
        price: f64,
    ) -> CreateOrder {
        CreateOrder {
            symbol: coins.to_string(),
            price,
            amount: match side {
                Side::Buy => amount,
                Side::Sell => -amount,
            },
            order_type: match target {
                Target::Market => "market".to_owned(),
                Target::Limit => "limit".to_owned(),
            },
        }
    }
}

pub type Markets = Vec<Market>;

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Market {
    pub id: String,
    pub base_unit: String,
    pub quote_unit: String,
    pub base_precision: i32,
    pub quote_precision: i32,
    pub display_precision: i32,
    pub price_change: f64,
}

#[derive(serde::Serialize, Clone, Debug)]
pub struct CancelOrderRequest {
    pub order_id: i32,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct CanceledOrderResponse {
    pub id: i32,
    pub side: String,
    #[serde(rename = "type")]
    pub order_type: String,
    pub price: String,
    pub avg_execution_price: String,
    pub state: String,
    pub symbol: String,
    pub timestamp: i32,
    pub original_amount: String,
    pub remaining_amount: String,
    pub executed_amount: String,
    pub is_cancelled: Option<String>,
    pub is_hidden: Option<String>,
    pub is_live: Option<String>,
    pub was_forced: Option<String>,
    pub exchange: Option<String>,
}
