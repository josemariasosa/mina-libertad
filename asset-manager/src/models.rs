use std::fmt;
use std::str::FromStr;
use serde::{Serialize, Deserialize};
use crate::PRICE_SHEET_FILEPATH;
use crate::types::{EpochMillis, FundName};

use crate::transaction::Transaction;
use crate::utils::now::Now;
use crate::utils::{normal_input_string, parse_option_u64};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum FiatCurrency {
    MXN,
    USD
}

impl FromStr for FiatCurrency {
    type Err = ();

    fn from_str(input: &str) -> Result<FiatCurrency, Self::Err> {
        println!("HERE: {}", input);
        let result = match normal_input_string(&input).as_str() {
            "MXN" => FiatCurrency::MXN,
            "USD" => FiatCurrency::USD,
            _ => unimplemented!()
        };
        return Ok(result)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum AppEnv {
    Dev,
    Prod
}

impl fmt::Display for AppEnv {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            AppEnv::Dev => write!(f, "dev"),
            AppEnv::Prod => write!(f, "prod"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Buy {
    transaction: Transaction,
    pub settled_at: EpochMillis
}

impl Buy {
    pub(crate) fn new(
        settled_at: EpochMillis,
        transaction_str: String,
        amount: u128,
        currency: FiatCurrency
    ) -> Self {
        match transaction_str.to_lowercase().as_str() {
            "fiat_cash" => {
                Buy {
                    transaction: Transaction::FiatCash { amount, currency },
                    settled_at
                }
            },
            _ => panic!(),
        }
        
    }

    pub(crate) fn get_transaction_amount_currency(&self) -> (u128, FiatCurrency) {
        match self.transaction.clone() {
            Transaction::FiatCash { amount, currency } => (amount, currency)
        }
    }

    pub(crate) fn get_entrance_amount(&self, user_fiat_currency: FiatCurrency) -> u128 {
        match self.transaction.clone() {
            Transaction::FiatCash { amount, currency } => {
                if currency == user_fiat_currency {
                    amount
                } else {
                    unimplemented!();
                }
            }
        }
    }

    // pub(crate) fn get_settled_at(&self) -> EpochMillis {
    //     self.settled_at
    // }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Sell {
    transaction: Transaction,
    settled_at: EpochMillis
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Fund {
    name: String,
    location: Option<String>
}

impl Fund {
    pub fn new(name: FundName) -> Self {
        Fund {
            name,
            location: None
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MarketSnapshot {
    timestamp: EpochMillis,
    asset_type_str: String,
    source: Option<String>,
    currency: FiatCurrency,
    market: Option<String>,
    top: Option<u64>,
    bottom: Option<u64>,
    median: u64
}

impl MarketSnapshot {
    pub fn new(
        timestamp: EpochMillis,
        asset_type_str: String,
        source: Option<String>,
        currency: FiatCurrency,
        market: Option<String>,
        top: Option<u64>,
        bottom: Option<u64>,
        median: u64
    ) -> Self {
        MarketSnapshot {
            timestamp,
            asset_type_str,
            source,
            currency,
            market,
            top,
            bottom,
            median
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Use u64 because for Default fiat currency only consider 2 decimals.
pub struct PriceSheet {
    pub gold_gram_24k: Option<u64>,
    pub gold_gram_21k: Option<u64>,
    pub btc: Option<u64>,
    pub doge_4_decimals: Option<u64>,
    pub ltc: Option<u64>,
    pub eth: Option<u64>,
    pub created_at: EpochMillis
}

impl Default for PriceSheet {
    fn default() -> Self {
        let price_sheet = json::parse(&PRICE_SHEET_FILEPATH).unwrap();
        PriceSheet {
            gold_gram_24k: parse_option_u64(&price_sheet, "gold_gram_24k"),
            gold_gram_21k: parse_option_u64(&price_sheet, "gold_gram_21k"),
            btc: parse_option_u64(&price_sheet, "btc"),
            doge_4_decimals: parse_option_u64(&price_sheet, "doge_4_decimals"),
            ltc: parse_option_u64(&price_sheet, "ltc"),
            eth: parse_option_u64(&price_sheet, "eth"),
            created_at: Now::new().to_epoch_millis()
        }
    }
}