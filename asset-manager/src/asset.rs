use std::fmt;

// use std::fmt;
use json::JsonValue;
use serde::{Serialize, Deserialize};
use crate::{types::{AssetId, EpochMillis}, models::{Fund, Buy, Sell, FiatCurrency, MarketSnapshot, PriceSheet}, utils::{now::Now, parse_option_string, parse_option_u16, proportional}, user::UserSettings};

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

    fn naive_market_price(&self, use_price_sheet: PriceSheet) -> MarketSnapshot {
        println!("{}", self);
        println!("{:?}", use_price_sheet);
        let median = match &self {
            AssetType::Bitcoin { address, sats } => {
                // (10k each btc) 10_000_00 * 3_7000_0000 / (magic) 100_000_000
                proportional(
                    use_price_sheet.btc.unwrap() as u128,
                    sats.clone(),
                    10u128.pow(8)
                )
            },
            AssetType::Gold { presentation, weight, purity, note }  => {
                let weight = weight.as_ref().unwrap().parse::<u128>().unwrap();
                let price = match purity.unwrap() {
                    9999 => use_price_sheet.gold_gram_24k.unwrap(),
                    9000 => use_price_sheet.gold_gram_21k.unwrap(),
                    _ => unimplemented!()
                };
                proportional(price as u128, weight, 1_u128)
            },
            AssetType::Litecoin { address, lits } => {
                proportional(
                    use_price_sheet.ltc.unwrap() as u128,
                    lits.clone(),
                    10u128.pow(8)
                )
            },
            AssetType::Ethereum { address, wei } => {
                proportional(
                    use_price_sheet.eth.unwrap() as u128,
                    wei.clone(),
                    10u128.pow(18)
                )
            },
            AssetType::Dogecoin { address, dogs } => {
                proportional(
                    use_price_sheet.doge_4_decimals.unwrap() as u128,
                    dogs.clone(),
                    10u128.pow(10)
                )
            },
            AssetType::RealState { name, deed_date } => {
                2_000_000_u128
            },
        };
        MarketSnapshot::new(
            Now::new().to_epoch_millis(),
            format!("{}", &self),
            None,
            FiatCurrency::MXN,
            None,
            None,
            None,
            median as u64
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
    pub id: AssetId,
    fund: Fund,
    asset_type: AssetType,
    buy: Option<Buy>,
    sell: Option<Sell>,
    owner_settings: UserSettings
}

impl Asset {
    pub fn new(id: AssetId, fund: Fund, asset_type: AssetType, owner_settings: UserSettings) -> Self {
        Asset {
            id,
            fund,
            asset_type,
            buy: None,
            sell: None,
            owner_settings
        }
    }

    pub fn purchase(&mut self, settled_at: EpochMillis, amount: u128, currency: FiatCurrency) {
        assert!(self.buy.is_none(), "Asset already has a Buy process.");
        let buy = Buy::new(settled_at, "fiat_cash".to_string(), amount, currency);
        self.buy = Some(buy);
    }

    /// Entrance amount is the SINGLE total value the user paid. Expect fiat currancy.
    fn get_entrance_amount(&self) -> u128 {
        assert!(self.buy.is_some());
        self.buy
            .as_ref()
            .unwrap()
            .get_entrance_amount(self.owner_settings.fiat_currency.clone())
    }

    pub fn evaluate(&self, currency: FiatCurrency) -> AssetEvaluation {
        let buy_settled_at = self.buy.as_ref().expect("Asset without a Buy object.").settled_at;
        AssetEvaluation {
            asset_id: self.id,
            millisec_since_purchase: Now::get_millis_since(buy_settled_at),
            asset_type_str: format!("{}", self.asset_type),
            entrance_amount: self.get_entrance_amount(),
            now_amount: self.get_market_price(Some(self.price_sheet)), // TODO: First get the market update.
            currency: self.owner_settings.fiat_currency.clone()
        }
    }

    // TODO: implement other ways to get the price
    pub fn get_market_price(&self, use_price_sheet: Option<PriceSheet>) -> MarketSnapshot {
        self.asset_type.naive_market_price(use_price_sheet.unwrap())
    }
}
