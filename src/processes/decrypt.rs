use crate::{ProcessConfig, utility};

use super::justify_keys::determine_keys;


pub fn decrypt(config: &mut ProcessConfig) {
    determine_keys(config);
     config.flush_keys();
     
    soft_decrypt(config);
}

pub fn soft_decrypt(config: &mut ProcessConfig) {

    let split_chunks: Vec<String> =
        utility::perform::split_string_into_chunks(&config.read_blur, config.user_key_length);
        
    let mut result = String::new();

    for chunk in split_chunks {
        result.push_str(config.process_soft_bundle.get(&chunk).unwrap());
    }
 
    config.user_clear_payload = result;
}
