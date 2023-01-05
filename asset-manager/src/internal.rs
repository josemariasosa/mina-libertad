
use std::fs;
use std::path::Path;

use serde::{Serialize, Deserialize};
use crate::app::App;
use crate::types::{UserName, HashString, FundName};
use crate::models::{AppEnv, Fund};
// use crate::errors::AppErrors;
// use std::fs;
// use std::path::Path;

impl App {
    pub(crate) fn internal_get_fund(&mut self, name: &FundName) -> Fund {
        self.funds.get(name).expect("Fund not found!").clone()
    }
}