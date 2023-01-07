use std::fs;
use std::str::FromStr;
 
use crate::models::{Fund, FiatCurrency, MarketSnapshot, PriceSheet};
use crate::asset::{Asset, AssetType, AssetEvaluation};
use crate::types::{FundName, AssetId};
use crate::user::{User, UserSettings};
use crate::utils::now::Now;
use json::JsonValue;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct App {
    owner: User,
    user_settings: UserSettings,
    next_asset_id: u32,
    pub funds: HashMap<FundName, Fund>,
    assets: Vec<Asset>,
    pub price_sheet: PriceSheet,
    /// TODO: key, String, is the asset_type!!!
    latest_prices: HashMap<AssetId, MarketSnapshot>
}

impl App {
    pub(crate) fn new(owner: User) -> Self {
        App {
            owner,
            user_settings: UserSettings::default(),
            next_asset_id: 0,
            funds: HashMap::new(),
            assets: Vec::new(),
            price_sheet: PriceSheet::default(),
            latest_prices: HashMap::new()
        }
    }

    fn create_funds(&mut self, funds: JsonValue) {
        for fund in funds.members() {
            let fund_name = fund["name"].to_string();
            if let Some(_) = self.funds.get(&fund_name) {
                panic!("Fund already exist");
            }
            self.funds.insert(
                fund_name.clone(), Fund::new(fund_name)
            );
        }
    }

    fn create_assets(&mut self, assets: JsonValue) {
        for asset in assets.members() {
            let fund = self.internal_get_fund(&asset["fund"]["name"].to_string());
            let asset_type = AssetType::new(
                asset["asset_type"]["type"].to_string(),
                asset["asset_type"]["data"].clone()
            );

            let mut new_asset = Asset::new(
                self.next_asset_id,
                fund.clone(),
                asset_type,
                self.user_settings.clone()
            );
            self.next_asset_id += 1;

            if !asset["buy"].is_null() {
                let settled_at = Now::new_from_datetime_str(
                    asset["buy"]["settled_at"].as_str().unwrap(),
                    "%Y-%m-%d"
                ).to_epoch_millis();
                let amount = asset["buy"]["transaction"]["fiat_cash"]["amount"]
                    .to_string()
                    .parse::<u128>()
                    .unwrap();
                let currency = FiatCurrency::from_str(
                    asset["buy"]["transaction"]["fiat_cash"]["currency"].as_str().unwrap()
                ).unwrap();
                new_asset.purchase(settled_at, amount, currency);
            }
            self.assets.push(new_asset);
        }
    }

    pub(crate) fn update_market(&mut self, file_path: &str) {
        let content = fs::read_to_string(file_path).expect("Error reading market file.");
        let mut _market_json = json::parse(&content).unwrap();

        for asset in self.assets.iter() {
            let snapshot = asset.get_market_price(Some(&self.price_sheet));
            self.latest_prices.insert(asset.id, snapshot);
            
            // // TODO: unimplemented()
            // _market_json.push(serde_json::to_string(&snapshot).unwrap());
        }
    }

    pub(crate) fn import_user_file(&mut self, file_path: &str) {
        let content = fs::read_to_string(file_path).expect("Error reading assets file.");
        let user_json = json::parse(&content).unwrap();

        // notimplemented
        // let _cash_positions = user_json["cash_positions"].clone();

        let funds = user_json["funds"].clone();
        self.create_funds(funds);

        let assets = user_json["assets"].clone();
        self.create_assets(assets);



        // println!("{:?} 👺", self.assets)
    
        // let res = Vec::<Asset>::new();
        // for asset in assets.members() {
        //     let fund = Fund::new(asset["fund"]["name"].to_string());
        //     let new = Asset::new(self.next_asset_id, fund, asset_type)
        // }
    }

    /// Asset Id | Type | Entrance | Now
    fn view_asset_eval(&self) {
        let assets: Vec<AssetEvaluation> = self.assets.iter().map(|asset| asset.evaluate(FiatCurrency::MXN)).collect();

        println!("FRIDAY -> print the evaluated assets: {:#?}", assets);

    }

    pub(crate) fn dashboard(&self) {
        self.view_asset_eval();
        // self.view_entrance_point();

    }
}
