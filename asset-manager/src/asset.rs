use std::fmt;

// use std::fmt;
use json::JsonValue;
use serde::{Serialize, Deserialize};
use crate::{types::{AssetId, EpochMillis}, models::{Fund, Buy, Sell, FiatCurrency}, utils::{now::Now, parse_option_string, parse_option_u16}};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum AssetType {
    Gold { presentation: String, weight: Option<String>, purity: Option<u16>, note: Option<String> },
    Bitcoin { address: Option<String>, sats: u128 },
    Litecoin { address: Option<String>, lits: u128 },
    Ethereum { address: Option<String>, wei: u128 },
    Dogecoin { address: Option<String>, dogs: u128 },
    RealState { name: String, deed_date: Option<String> },
}

impl fmt::Display for AssetType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            AssetType::Gold { presentation, weight, purity, note } => "GOLD",
            AssetType::Bitcoin { address, sats } => "BTC",
            AssetType::Dogecoin { address, dogs } => "DOGE",
            AssetType::Litecoin { address, lits } => "LTC",
            AssetType::Ethereum { address, wei } => "ETH",
            AssetType::RealState { name, deed_date } => "REAL_STATE"
        })
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

pub struct AssetEvaluation {
    asset_id: AssetId,
    millisec_since_purchase: EpochMillis,
    asset_type_str: String,
    entrance_amount: u128,
    now_amount: u128,
    currency: FiatCurrency
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

    fn get_entrance_amount(&self) -> u128 {
        assert!(self.buy.is_none());
    }

    pub fn evaluate(&self, currency: FiatCurrency) -> AssetEvaluation {
        AssetEvaluation {
            asset_id: self.id,
            millisec_since_purchase: Now::new().to_epoch_millis() - self.buy.expect("Asset without a Buy object.").settled_at,
            asset_type_str: format!("{}", self.asset_type),
            entrance_amount: self.get_entrance_amount(),
            now_amount: u128,
            currency: FiatCurrency
        }
    }
}
