// use std::fmt;
use json::JsonValue;
use serde::{Serialize, Deserialize};
use crate::{types::{AssetId, EpochMillis}, models::{Fund, Buy, Sell, FiatCurrency}};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum AssetType {
    Gold { presentation: String, weight: Option<String>, purity: Option<u16>, note: Option<String> },
    Bitcoin { address: Option<String>, sats: u128 },
    Litecoin { address: Option<String>, lits: u128 },
    Ethereum { address: Option<String>, wei: u128 },
    Dogecoin { address: Option<String>, dogs: u128 },
    RealState { name: String, deed_date: Option<String> },
}

fn parse_option_string(data: &JsonValue, key: &str) -> Option<String> {
    if data[key].is_null() {
        None
    } else {
        Some(data[key].to_string())
    }
}

fn parse_option_u16(data: &JsonValue, key: &str) -> Option<u16> {
    if data[key].is_null() {
        None
    } else {
        Some(data[key].as_u16().unwrap())
    }
}


impl AssetType {
    pub fn new(type_str: String, data: JsonValue) -> Self {
        match type_str.to_lowercase().as_str() {
            "bitcoin" => {
                Self::Bitcoin {
                    address: parse_option_string(&data, "address"),
                    sats: data["sats"].to_string().parse::<u128>().unwrap()
                }
            },
            "litecoin" => {
                Self::Litecoin {
                    address: parse_option_string(&data, "address"),
                    lits: data["lits"].to_string().parse::<u128>().unwrap()
                }
            },
            "ethereum" => {
                Self::Ethereum {
                    address: parse_option_string(&data, "address"),
                    wei: data["wei"].to_string().parse::<u128>().unwrap()
                }
            },
            "dogecoin" => {
                Self::Dogecoin {
                    address: parse_option_string(&data, "address"),
                    dogs: data["dogs"].to_string().parse::<u128>().unwrap()
                }
            },
            "real_state" => {
                Self::RealState {
                    deed_date: parse_option_string(&data, "deed_date"),
                    name: data["name"].to_string()
                }
            },
            "gold" => {
                Self::Gold {
                    presentation: data["presentation"].to_string(),
                    weight: parse_option_string(&data, "weight"),
                    purity: parse_option_u16(&data, "purity"),
                    note: parse_option_string(&data, "note")
                }
            },
            _ => {
                // TODO: Create a new Default Other Asset
                unimplemented!();
            }
        }

    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Asset {
    id: AssetId,
    fund: Fund,
    asset_type: AssetType,
    buy: Option<Buy>,
    sell: Option<Sell>,
}

impl Asset {
    pub fn new(id: AssetId, fund: Fund, asset_type: AssetType) -> Self {
        Asset {
            id,
            fund,
            asset_type,
            buy: None,
            sell: None
        }
    }

    pub fn purchase(&mut self, settled_at: EpochMillis, amount: u128, currency: FiatCurrency) {
        assert!(self.buy.is_none(), "Asset already has a Buy process.");
        let buy = Buy::new(settled_at, "fiat_cash".to_string(), amount, currency);
        self.buy = Some(buy);
    }
}
