use std::collections::HashMap;

use crate::{utility, ProcessConfig, processes::{self, softcrypt::decrypt}};

pub fn direct_encrypt(input_string: String, key_size: usize) -> (String, String) {
    if input_string.is_empty() || key_size == 0 { panic!("Content is empty. Breaking process")}

    let mut config: ProcessConfig = ProcessConfig {
        user_clear_payload: input_string,
        user_key_length: key_size,
        process_chosen_index: 0, // Encrypt process
        
        system_synapse: utility::generate::random_string(4),

        process_soft_bundle: HashMap::new(),
        process_created_blur: String::new(),

        read_blur: String::new(),
        read_keys: String::new(),
    };

    

    processes::softcrypt::encrypt(&mut config);

    for (key, value) in config.process_soft_bundle.iter() {
        let line = format!("{}{}{}{}", key, config.system_synapse, value, config.system_synapse); 
        
        config.read_keys.push_str(&line);
    }

    (config.process_created_blur, config.read_keys)
}

pub fn direct_decrypt(provisioned_brick: String, provisioned_keys: String) -> String {
    if provisioned_brick.is_empty() || provisioned_keys.is_empty() { panic!("Content is empty. Breaking process")}

    let mut config: ProcessConfig = ProcessConfig {
        user_clear_payload: String::new(),
        user_key_length: 0,
        process_chosen_index: 1, // Encrypt process
        
        system_synapse: String::new(),

        process_soft_bundle: HashMap::new(),
        process_created_blur: String::new(),

        read_blur: provisioned_brick,
        read_keys: provisioned_keys,
    };

    decrypt(&mut config);

    config.user_clear_payload
}