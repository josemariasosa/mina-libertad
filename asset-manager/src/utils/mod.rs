pub(crate) mod now;

fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}

pub(crate) fn normal_input_string(s: &str) -> String {
    let mut res = s.to_uppercase();
    remove_whitespace(&mut res);
    res
}