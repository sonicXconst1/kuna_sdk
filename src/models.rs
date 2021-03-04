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
    pub fn with(
        coins: crate::coin::Coins,
        raw_order_book: OrderBookEntries
    ) -> OrderBook {
        let (bids, asks) = raw_order_book
            .into_iter()
            .fold(
                (Vec::new(), Vec::new()),
                |mut acc, entry| {
                    if entry.1 > 0.0 {
                        acc.0.push(OrderBookItem::from(entry))
                    } else {
                        acc.1.push(OrderBookItem::from(entry))
                    }
                    acc
                });
        OrderBook {
            coins,
            bids,
            asks
        }
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
