use crate::ProcessConfig;
use crate::utility;



pub fn encrypt(config: &mut ProcessConfig) {
    soft_encrypt(config);
}

pub fn soft_encrypt(config: &mut ProcessConfig) {
    let mut known_entries: Vec<char> = vec![];
    
    for letter in config.user_clear_payload.chars() {
        
        if !known_entries.contains(&letter) {
            config.process_soft_bundle.insert(letter.to_string(), utility::generate::random_string(config.user_key_length));
            known_entries.push(letter);
        } else {
            
        }
        config.process_blur_payload.push_str(config.process_soft_bundle.get(&letter.to_string()).unwrap());
        
    }
}