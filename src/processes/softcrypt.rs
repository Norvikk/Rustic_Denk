use crate::{ProcessConfig, utility};
use std::collections::HashSet;
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




pub fn encrypt(config: &mut ProcessConfig) {
    soft_encrypt(config);
}



pub fn soft_encrypt(config: &mut ProcessConfig) {
    let mut known_entries: HashSet<char> = HashSet::new();
    let mut known_iterations: HashSet<String> = HashSet::new();

    let max_regenerations: usize = 100; // Maximum number of regenerations
    let mut regenerations: usize = 0; // Counter for regeneration attempts

    for letter in config.user_clear_payload.chars() {
        if known_entries.insert(letter) {
            let generated_iteration = loop {
                let iteration = utility::generate::random_string(config.user_key_length);
                if known_iterations.insert(iteration.clone()) {
                    break iteration;
                }

                regenerations += 1;
                if regenerations >= max_regenerations {
                    panic!("Maximum regeneration attempts reached - Unable to generate a unique iteration.");
                }
            };

            regenerations = 0; // Reset the regeneration counter
            config.process_created_blur.push_str(&generated_iteration);
            config.process_soft_bundle.insert(letter.to_string(), generated_iteration);
            
        }
        else {
             config.process_created_blur.push_str(config.process_soft_bundle.get(&letter.to_string()).unwrap());
        }
        
        
    }
}
