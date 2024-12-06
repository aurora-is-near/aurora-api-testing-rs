extern crate serde;
extern crate serde_derive;
extern crate serde_json;

pub fn hex_string_to_i32(s: String) -> i32 {
    i32::from_str_radix(&s[2..s.len()], 16).unwrap()
}

pub fn hex_string_to_i64(s: String) -> i64 {
    i64::from_str_radix(&s[2..s.len()], 16).unwrap()
}
