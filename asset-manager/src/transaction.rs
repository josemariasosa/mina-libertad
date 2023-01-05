use serde::{Serialize, Deserialize};
use crate::models::FiatCurrency;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Transaction {
    FiatCash {
        amount: u128,
        currency: FiatCurrency
    }
}