use crate::ProcessConfig;
use crate::utility;
use std::collections::HashSet;



pub fn encrypt(config: &mut ProcessConfig) {
    soft_encrypt(config);
}



pub fn soft_encrypt(config: &mut ProcessConfig) {
    let mut known_entries: HashSet<char> = HashSet::new();
    
    for letter in config.user_clear_payload.chars() {
        if known_entries.insert(letter) {
            config.process_soft_bundle.insert(letter.to_string(), utility::generate::random_string(config.user_key_length));
        }
        config.process_created_blur.push_str(config.process_soft_bundle.get(&letter.to_string()).unwrap());
    }
}
