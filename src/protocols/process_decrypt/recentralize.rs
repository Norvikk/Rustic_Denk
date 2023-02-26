pub fn revert_ascii_by_10(input: &str) -> String {
    let mut result = String::new();
    for c in input.chars() {
        let ascii_val = c as u8;
        let new_ascii_val = ascii_val.wrapping_sub(50);
        let new_char = new_ascii_val as char;
        result.push(new_char);
    }
    result
}