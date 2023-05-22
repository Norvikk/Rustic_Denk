use std::collections::HashMap;

use crate::{utility, ProcessConfig, processes};

pub fn encrypt(input_string: String, key_size: usize, synapse_size: Option<usize>) -> (String, HashMap<String, String>) {
    if input_string.is_empty() || key_size == 0 { panic!("Content is empty. Breaking process")}

    let mut config: ProcessConfig = ProcessConfig {
        user_clear_payload: input_string,
        user_key_length: key_size,
        process_chosen_index: synapse_size.unwrap_or(10), // Encrypt process
        
        system_synapse: utility::generate::random_string(10),

        process_soft_bundle: HashMap::new(),
        process_created_blur: String::new(),

        read_blur: String::new(),
        read_keys: String::new(),
    };

    processes::encrypt::encrypt(&mut config);

    (config.process_created_blur, config.process_soft_bundle)
}