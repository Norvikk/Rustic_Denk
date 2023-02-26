use std::fs;


use super::{process_encrypt::{key_encryption::EncryptionKey, key_buffering::BufferKey}, process_decrypt::{self}};

pub fn run() -> anyhow::Result<String> {
    let keys_path = get_keys_path();
    let bricked_path = get_bricked_path();

    let keys_content: String;
    let mut bricked_content: String;

    match read_keys_path(&keys_path){
        Ok(result) => keys_content = result,
        Err(err) => panic!("Key file path provided <{}> caused this error: {}", keys_path,err)
    }

    match read_bricked_path(&bricked_path){
        Ok(result) => bricked_content = result,
        Err(err) => panic!("Bricked file path provided <{}> caused this error: {}", keys_path,err)
    }

    let keys_chunks: Vec<&str> = keys_content.split("BUFFER").collect();

    let buffer_chunk = keys_chunks[0].to_string();
    
    let vanilla_chunk = keys_chunks[1].to_string();


    let carrier: Vec<&str> = vanilla_chunk.split("BINDING").collect();
    let binding_chunk = carrier[1].to_string();


    

    bricked_content = process_decrypt::recentralize::revert_ascii_by_10(&bricked_content);

    let vanilla_keys: Vec<EncryptionKey> = process_decrypt::vanilla::determine_keys(vanilla_chunk)?;
    
    let buffer_keys: Vec<BufferKey> = process_decrypt::debuffer::determine_keys(buffer_chunk)?;
    

    let buffer_package = process_decrypt::unbind::unbind( &binding_chunk, &bricked_content);

    let debuffered = process_decrypt::debuffer::determine_payload(buffer_keys, &buffer_package)?;
    let devanilla = process_decrypt::vanilla::determine_payload(vanilla_keys, &debuffered)?;


    Ok(devanilla)
}

fn get_keys_path() -> String {
    let keys_path = inquire::Text::new("Keys:").with_placeholder("/src, C:/Users/Administrator/Downloads").with_default("").with_help_message("Enter the directory where the KEYS file is stored").prompt().unwrap();

    if keys_path.is_empty() {
        return "keys.dnk".to_string();
    } else {
        return format!("{}/keys.dnk", keys_path).to_string();
    }
}

fn get_bricked_path() -> String {
    let keys_path = inquire::Text::new("Bricked:").with_placeholder("/src, C:/Users/Administrator/Downloads").with_default("").with_help_message("Enter the directory where the BRICKED file is stored").prompt().unwrap();

    if keys_path.is_empty() {
        return "bricked.dnk".to_string();
    } else {
        return format!("{}/bricked.dnk", keys_path).to_string();
    }
}

fn read_keys_path(path: &str) -> anyhow::Result<String>{
    Ok(fs::read_to_string(&path)?)
}

fn read_bricked_path(path: &str) -> anyhow::Result<String>{
    Ok(fs::read_to_string(&path)?)
}