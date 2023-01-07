use json::JsonValue;
use uint::construct_uint;

construct_uint! {
    /// 256-bit unsigned integer.
    pub struct U256(4);
}

pub(crate) mod now;

fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}

pub(crate) fn normal_input_string(s: &str) -> String {
    let mut res = s.to_uppercase();
    remove_whitespace(&mut res);
    res
}

pub fn parse_option_string(data: &JsonValue, key: &str) -> Option<String> {
    if data[key].is_null() {
        None
    } else {
        Some(data[key].to_string())
    }
}

pub fn parse_option_u16(data: &JsonValue, key: &str) -> Option<u16> {
    if data[key].is_null() {
        None
    } else {
        Some(data[key].as_u16().unwrap())
    }
}

pub fn parse_option_u64(data: &JsonValue, key: &str) -> Option<u64> {
    if data[key].is_null() {
        None
    } else {
        Some(data[key].as_u64().unwrap())
    }
}

#[inline]
/// returns amount * numerator/denominator
pub fn proportional(amount: u128, numerator: u128, denominator: u128) -> u128 {
    return (U256::from(amount) * U256::from(numerator) / U256::from(denominator)).as_u128();
}