pub fn hex_string_to_i32(s: String) -> i32 {
    i32::from_str_radix(&s[2..s.len()], 16).unwrap()
}
