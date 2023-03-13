use crate::protocols::process_encrypt::decentralization::decentralize;



pub fn recentralize(text: &str, shift: u8) -> String {
    decentralize(text, 26 - shift)
}