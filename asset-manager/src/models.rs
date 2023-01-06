use std::fmt;
use std::str::FromStr;
use serde::{Serialize, Deserialize};
use crate::types::{EpochMillis, FundName};

use crate::transaction::Transaction;
use crate::utils::normal_input_string;

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
        self.transaction.amount, self.transaction.
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
