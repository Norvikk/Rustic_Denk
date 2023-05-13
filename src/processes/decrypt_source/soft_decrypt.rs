use crate::{ProcessConfig, utility};

pub fn soft_decrypt(config: &mut ProcessConfig) {
    let split_chunks: Vec<String> = utility::perform::split_string_into_chunks(&config.read_blur, config.user_key_length);
    let mut result = String::new();

    

    for chunk in split_chunks {
        println!("{}", chunk);
        result.push_str(config.process_soft_bundle.get(&chunk).unwrap());
    }

    
}