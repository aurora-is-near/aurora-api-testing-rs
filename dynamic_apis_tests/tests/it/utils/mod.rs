extern crate serde;
extern crate serde_derive;
extern crate serde_json;

#[allow(clippy::needless_pass_by_value)]
pub fn hex_string_to_i32(s: String) -> i32 {
    i32::from_str_radix(&s[2..s.len()], 16).unwrap()
}

#[allow(clippy::needless_pass_by_value)]
pub fn hex_string_to_i64(s: String) -> i64 {
    i64::from_str_radix(&s[2..s.len()], 16).unwrap()
}
