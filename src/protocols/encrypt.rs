use core::panic;
use inquire::validator::Validation;
use std::fs::File;
use std::io::prelude::*;

use super::process_encrypt;
use super::process_encrypt::binding::{bind, BindingKey};
use super::process_encrypt::key_buffering::BufferKey;
use super::process_encrypt::key_encryption::EncryptionKey;

pub fn run(is_burst: bool, size_key: i64) -> anyhow::Result<String> {
    let user_plain_text: String;
    let user_bricked_path: String;
    let user_keys_path: String;

    let mut break_decentralization_process: bool = false;

    if is_burst {
        user_plain_text = String::from("running test: a1+#-? öäüß");
        (user_bricked_path, user_keys_path) = request_write_path(("bricked", "keys"), ".dnk", true)?;
    } else {
        user_plain_text = request_plain_text()?;
        (user_bricked_path, user_keys_path) = request_write_path(("bricked", "keys"), ".dnk", false)?;
    }

    let vanilla_encrypted_bundle = process_encrypt::key_encryption::start(&user_plain_text, size_key);

    let buffered_encrypted_bundle = process_encrypt::key_buffering::start(&vanilla_encrypted_bundle);
    let mut binded_encrypted_bundle = bind(&buffered_encrypted_bundle.1, buffered_encrypted_bundle.0[0].key.len() as i64);

    let decentralization_cancel_triggers: Vec<char> = "!$%€&'()*,./;<=>@[\\]^_`{|}~¡¢£¤¥¦§¨©ª«¬®¯°±²³´µ¶·¸¹º»¼½¾¿×÷".chars().collect();

    for current_entry in user_plain_text.chars() {
        if decentralization_cancel_triggers.contains(&current_entry) {
            break_decentralization_process = true;
            println!("\nThe given context had unstable symbols therefore the DECENTRALIZATION feature has been disabled.\nUpon decrypting, please use option 2 (no decentralization).\n");
            continue;
        }
    }

    if !break_decentralization_process {
        binded_encrypted_bundle.1 = process_encrypt::decentralization::decentralize(binded_encrypted_bundle.1);
    }
    

    match write_files_to_dir(
        binded_encrypted_bundle,
        buffered_encrypted_bundle,
        vanilla_encrypted_bundle.0,
        user_bricked_path,
        user_keys_path,
        break_decentralization_process,
    ) {
        Ok(_) => Ok(String::from("Success converting.")),
        Err(err) => panic!("Critical error {} while encrypting", err),
    }
}

fn request_plain_text() -> anyhow::Result<String> {
    let validator = |input: &str| {
        if input.chars().count() < 2 {
            Ok(Validation::Invalid(
                "You must encrypt AT LEAST 2 characters.".into(),
            ))
        } else {
            Ok(Validation::Valid)
        }
    };

    let user_payload = inquire::Text::new("To Encrypt: ")
        .with_validator(validator)
        .with_help_message("Write here something you want to encrypt")
        .with_placeholder("123 asd #+_ äöü - anything")
        .prompt();

    Ok(user_payload?)
}
fn request_write_path(
    file_names: (&str, &str),
    file_format: &str,
    is_burst: bool,
) -> anyhow::Result<(String, String)> {
    let path;
    if is_burst {
        path = String::from("target");
    } else {
        path = inquire::Text::new("File Directory: ")
            .with_help_message(
                "Leaving this empty writes the output to this folder where the app is located",
            )
            .with_placeholder("Sample: C:/Users/Administrator/Downloads")
            .prompt()?
            .trim()
            .to_owned();
    }

    if path.is_empty() {
        Ok((file_names.0.to_string() + file_format, file_names.1.to_string() + file_format))
    } else {
        Ok((
            path.clone() + "/" + file_names.0 + file_format,
            path + "/" + file_names.1 + file_format,
        ))
    }
}
fn write_files_to_dir(
    binded_payload: (Vec<BindingKey>, String),
    encrypted_payload: (Vec<BufferKey>, String),
    keys: Vec<EncryptionKey>,
    bricked_path: String,
    keys_path: String,
    break_decentralization: bool

) -> anyhow::Result<()> {
    let mut bricked_file: File = File::create(&bricked_path)?;
    let mut keys_file: File = File::create(&keys_path)?;
    let mut to_write_keys: String = String::new();

    bricked_file.write(binded_payload.1.as_bytes())?;

    let last_object_0 = encrypted_payload.0[encrypted_payload.0.len() - 1].key.clone();
    for object in encrypted_payload.0.iter() {
        let to_write: String;

        if object.key != last_object_0 {
            to_write = format!("{}?s§0-a{}?s§0-a", object.symbol.to_string(), object.key);
        } else {
            to_write = format!("{}?s§0-a{}", object.symbol.to_string(), object.key);
        }

        to_write_keys.push_str(&to_write);
    }

    to_write_keys.push_str("BUFFER");

    for object in keys.iter() {
        let to_write = format!("{}?s§0-a{}?s§0-a", object.symbol.to_string(), object.key);

        to_write_keys.push_str(&to_write);
    }

    to_write_keys.push_str("BINDING");

    let last_object_1 = binded_payload.0[binded_payload.0.len() - 1].key.clone();
    for object in binded_payload.0.iter() {
        let to_write: String;

        if object.key != last_object_1 {
            to_write = format!(
                "{}?s§0-a{}?s§0-a{}?s§0-a",
                object.symbol.0.to_string(),
                object.symbol.1.to_string(),
                object.key
            );
        } else {
            to_write = format!(
                "{}?s§0-a{}?s§0-a{}",
                object.symbol.0.to_string(),
                object.symbol.1.to_string(),
                object.key
            );
        }

        to_write_keys.push_str(&to_write);
    }

    

    if !break_decentralization {
        to_write_keys = process_encrypt::decentralization::decentralize(to_write_keys);
    }
    keys_file.write(to_write_keys.as_bytes())?;
    Ok(())
}
