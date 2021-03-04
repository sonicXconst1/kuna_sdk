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

impl std::fmt::Display for Coin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter, 
            "{}",
            match self {
                Coin::TON => "TON",
                Coin::USDT => "USDT",
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
