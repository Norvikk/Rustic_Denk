use crate::protocols::tooling::key_generation::unique_key;


pub fn run(user_text: &str, size_key: i64) -> (Vec<EncryptionKey>, String) {
    let keys = initialize_keys(user_text, size_key);

    return (keys.0, keys.1);
}

#[derive(Debug)]
pub struct EncryptionKey {
    pub symbol: char,
    pub key: String,
}



fn initialize_keys(user_text: &str, size_key: i64) -> (Vec<EncryptionKey>, String) {
    let mut encryption_keys: Vec<EncryptionKey> = vec![];
    let mut is_contained: bool;
    let mut bricked_message: String = String::new();

    for letter in user_text.chars() {
        is_contained = false;

        for current_key in encryption_keys.iter() {
            if letter == current_key.symbol {
                bricked_message.push_str(&current_key.key);
                is_contained = true;
            }
        }

        if !is_contained {
            let carrier1 = EncryptionKey {
                key: unique_key(size_key),
                symbol: letter,
            };

            bricked_message.push_str(&carrier1.key);

            encryption_keys.push(carrier1);
        }
    }


    return (encryption_keys, bricked_message);
}
