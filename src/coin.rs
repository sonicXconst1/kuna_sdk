#[derive(Clone, Debug)]
pub enum Coin {
    TON,
    USDT,
    Unknown(String)
}

impl From<&str> for Coin {
    fn from(coin: &str) -> Coin {
        match coin {
            "USDT" => Coin::USDT,
            "TON" => Coin::TON,
            other => Coin::Unknown(other.to_owned()),
        }
    }
}

impl From<Coin> for String {
    fn from(coin: Coin) -> String {
        match coin {
            Coin::TON => "TON".to_owned(),
            Coin::USDT => "USDT".to_owned(),
            Coin::Unknown(other) => other,
        }
    }
}
