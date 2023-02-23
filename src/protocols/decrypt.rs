use std::fs;

use super::process_encrypt::key_encryption::EncryptionKey;

pub fn run() -> anyhow::Result<String> {

    let bricked_path = retrieve_path(
        "bricked",
        "Enter the path where the bricked file is stored. (Empty = origin dir)",
    )?;
let bricked;
    match determine_bricked(&bricked_path) {
        Ok(value) => bricked = value,
        Err(error) => panic!(
            "There is no such file for BRICKED -> {} -> GIVEN PATH -> {}",
            error, bricked_path
        ),
    }

    let keys_path = retrieve_path(
        "keys",
        "Enter the path where the keys file is stored. (Empty = origin dir)",
    )?;

    
    let keys;

    match determine_keys(&keys_path) {
        Ok(value) => keys = value,
        Err(error) => panic!(
            "There is no such file for KEYS -> {} -> GIVEN PATH -> {}",
            error, keys_path
        ),
    }

    let decrypted_text = determine_payload(keys, &bricked)?;
    Ok(decrypted_text)
}

fn retrieve_path(name: &str, message: &str) -> anyhow::Result<String> {
    let path = inquire::Text::new(message).prompt()?.trim().to_owned();

    let format: &str = ".dnk";

    if path.is_empty() {
        Ok(name.to_string() + format)
    } else {
        Ok(path + "/" + name + format)
    }
}

fn determine_keys(path_to_keys: &str) -> anyhow::Result<Vec<EncryptionKey>> {
    let mut keys: Vec<EncryptionKey> = vec![];

    let contents = fs::read_to_string(path_to_keys)?;
    let content_split: Vec<&str> = contents.split("?s§0-a").collect();

    let mut jump = false;
    let mut jump_value = "";
    for n in 0..content_split.len() - 1 {
        if !jump {
            jump_value = content_split[n];
            jump = true;
        } else {
            keys.push(EncryptionKey {
                key: content_split[n].to_string(),
                symbol: jump_value.chars().next().unwrap(),
            });
            jump = false;
        }
    }
    Ok(keys)
}

fn determine_bricked(path_to_bricked: &str) -> anyhow::Result<String> {
    Ok(fs::read_to_string(path_to_bricked)?)
}

fn determine_payload(keys: Vec<EncryptionKey>, brick: &str) -> anyhow::Result<String> {
    let mut payload = String::new();
    let key_size = keys[0].key.len() as i64;
    let iterable = split_bricked(key_size, brick);

    for bricked in iterable {
        for key in keys.iter() {
            if key.key == bricked {
                payload.push(key.symbol);
            }
        }
    }

    fn split_bricked(keysize: i64, brick: &str) -> Vec<String> {
        let mut slices: Vec<String> = vec![];

        let mut looper = 0;
        let mut result = String::new();
        for char in brick.chars() {
            if looper == keysize {
                slices.push(result);
                result = char.to_string();
                looper = 1;
            } else {
                looper += 1;
                result.push(char);
            }
        }

        return slices;
    }

    Ok(payload)
}
