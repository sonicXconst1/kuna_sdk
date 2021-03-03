use std::str::FromStr;

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
        unimplemented!()
    }
}
