use crate::ProcessConfig;
use crate::utility;

pub fn soft_encrypt(config: &mut ProcessConfig) {
    let known_entries: Vec<char> = vec![];
    
    for letter in config.user_clear_payload.chars() {
        if !known_entries.contains(&letter) {
            config.process_soft_bundle.insert(letter.to_string(), utility::generate::random_string(true, true, true, true, config.user_key_length));
        }
        config.process_blur_payload.push_str(config.process_soft_bundle.get(&letter.to_string()).unwrap());
    }
}