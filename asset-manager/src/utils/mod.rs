use json::JsonValue;

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