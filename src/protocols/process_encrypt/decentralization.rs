pub fn decentralize(text: &str, shift: u8) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                let offset = (c as u8 - base + shift) % 26;
                (base + offset) as char
            } else {
                c
            }
        })
        .collect()
}
