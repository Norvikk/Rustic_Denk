use std::collections::HashMap;
use std::fs;

use crate::{ utility, ProcessConfig, processes::{self, softcrypt::decrypt, filecrypt::{encrypt_with_seed, string_to_u64, decrypt_with_seed}}};

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

pub fn direct_filecrypt(seed: String) {
    encrypt_with_seed(string_to_u64(&seed));  
}

pub fn direct_filedecrypt(seed: String) {
    decrypt_with_seed(string_to_u64(&seed));
}

pub fn create_folders() {
    let root_folder = "filecrypt";
    let subfolder_names = ["1_input_encrypt", "2_output_encrypt", "3_decrypted"];

    let textcrypt_folder = "textcrypt";

    if let Err(e) = fs::create_dir(textcrypt_folder) {
        if e.kind() != std::io::ErrorKind::AlreadyExists {
            eprintln!("Failed to create textcrypt folder '{}': {}", root_folder, e);
        }
    }

    if let Err(e) = fs::create_dir(root_folder) {
        if e.kind() != std::io::ErrorKind::AlreadyExists {
            eprintln!("Failed to create root folder '{}': {}", root_folder, e);
        }
    }

    

    for subfolder_name in &subfolder_names {
        let folder_path = format!("{}/{}", root_folder, subfolder_name);
        if let Err(e) = fs::create_dir(&folder_path) {
            if e.kind() != std::io::ErrorKind::AlreadyExists {
                eprintln!("Failed to create subfolder '{}': {}", folder_path, e);
            }
        }
    }
}