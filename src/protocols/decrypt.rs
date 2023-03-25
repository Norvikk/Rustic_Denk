use std::fs;

use super::{
    process_decrypt::{self, recentralize::{recentralize}},
    process_encrypt::{key_buffering::BufferKey, key_encryption::EncryptionKey},
};

pub fn start(is_burst: bool, is_decentralization: bool) -> anyhow::Result<String> {
    let user_keys_file_path: String;
    let user_bricked_file_path: String;
    let recentralized_read_file;
    let read_keys_content: String;
    let read_bricked_content: String;
    let recentralized;

    if is_burst {
        user_keys_file_path = request_keys_file_path(true);
        user_bricked_file_path = reqest_bricked_file_path(true);
    } else {
        user_keys_file_path = request_keys_file_path(false);
        user_bricked_file_path = reqest_bricked_file_path(false);
    }

    match read_keys_file_by_path(&user_keys_file_path) {
        Ok(result) => read_keys_content = result,
        Err(err) => panic!(
            "Key file path provided <{}> caused this error: {}",
            user_keys_file_path, err
        ),
    }

    match read_bricked_file_by_path(&user_bricked_file_path) {
        Ok(result) => read_bricked_content = result,
        Err(err) => panic!(
            "Bricked file path provided <{}> caused this error: {}",
            user_keys_file_path, err
        ),
    }
    

    if is_decentralization {
        recentralized_read_file = recentralize(&read_keys_content);
        recentralized = process_decrypt::recentralize::recentralize(&read_bricked_content);
    } else {
       recentralized_read_file = read_keys_content;
       recentralized = read_bricked_content;
    }

    let buffer_split_chunk: Vec<&str> = recentralized_read_file.split("BUFFER").collect();

    let buffer_chunk = buffer_split_chunk[0].to_string();
    let vanilla_chunk = buffer_split_chunk[1].to_string();

    let binding_chunk = vanilla_chunk.split("BINDING").collect::<Vec<&str>>()[1].to_string();

    let vanilla_keys: Vec<EncryptionKey> = process_decrypt::vanilla::determine_keys(vanilla_chunk)?;
    let buffer_keys: Vec<BufferKey> = process_decrypt::debuffer::determine_keys(buffer_chunk)?;

    

    

    let buffer_block = process_decrypt::unbind::unbind(
        &binding_chunk,
        &recentralized,
    );

    let debuffered = process_decrypt::debuffer::determine_payload(buffer_keys, &buffer_block)?;
    let vanilla_read = process_decrypt::vanilla::determine_payload(vanilla_keys, &debuffered)?;

    Ok(vanilla_read)
}

fn request_keys_file_path(burst: bool) -> String {
    let keys_path;
    if burst {
        keys_path = String::from("target");
    } else {
        keys_path = inquire::Text::new("Keys:")
            .with_placeholder("/src, C:/Users/Administrator/Downloads")
            .with_default("")
            .with_help_message("Enter the directory where the KEYS file is stored")
            .prompt()
            .unwrap();
    }

    if keys_path.is_empty() {
        return "keys.dnk".to_string();
    } else {
        return format!("{}/keys.dnk", keys_path).to_string();
    }
}

fn reqest_bricked_file_path(is_burst: bool) -> String {
    let keys_path;

    if is_burst {
        keys_path = String::from("target");
    } else {
        keys_path = inquire::Text::new("Bricked:")
            .with_placeholder("/src, C:/Users/Administrator/Downloads")
            .with_default("")
            .with_help_message("Enter the directory where the BRICKED file is stored")
            .prompt()
            .unwrap();
    }

    if keys_path.is_empty() {
        return "bricked.dnk".to_string();
    } else {
        return format!("{}/bricked.dnk", keys_path).to_string();
    }
}

fn read_keys_file_by_path(path: &str) -> anyhow::Result<String> {
    Ok(fs::read_to_string(&path)?)
}

fn read_bricked_file_by_path(path: &str) -> anyhow::Result<String> {
    Ok(fs::read_to_string(&path)?)
}
