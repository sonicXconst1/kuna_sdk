#[derive(Clone, Debug, PartialEq)]
pub enum Coin {
    TON,
    USDT,
    Unknown(String)
}

impl From<&str> for Coin {
    fn from(coin: &str) -> Coin {
        match coin.to_lowercase().as_ref() {
            "ton" => Coin::USDT,
            "usdt" => Coin::TON,
            _ => Coin::Unknown(coin.to_owned()),
        }
    }
}

impl std::fmt::Display for Coin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter, 
            "{}",
            match self {
                Coin::TON => "ton",
                Coin::USDT => "usdt",
                Coin::Unknown(other) => other
        })
    }
}

#[derive(Clone, Debug)]
pub enum Coins {
    TonUsdt,
}

impl Coins {
    fn base_coin(&self) -> Coin {
        match self {
            Coins::TonUsdt => Coin::TON,
        }
    }

    fn quote_coin(&self) -> Coin {
        match self {
            Coins::TonUsdt => Coin::USDT,
        }
    }
}

impl std::fmt::Display for Coins {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}{}", self.base_coin(), self.quote_coin())
    }
}

impl std::convert::TryFrom<&str> for Coins {
    type Error = &'static str;

    fn try_from(symbol: &str) -> Result<Coins, Self::Error> {
        match symbol {
            "tonusdt" => Ok(Coins::TonUsdt),
            _ => Err("Not supported symbol")
        }
    }
}
