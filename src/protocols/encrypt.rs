use core::panic;
use inquire::validator::Validation;
use std::fs::File;
use std::io::prelude::*;

use super::process_encrypt;
use super::process_encrypt::binding::{bind, BindingKey};
use super::process_encrypt::key_buffering::BufferKey;
use super::process_encrypt::key_encryption::EncryptionKey;

pub fn run(burst: bool, size_key: i64) -> anyhow::Result<String> {
    let payload: String;
    let bricked_path: String;
    let keys_path: String;

    if burst {
        payload = String::from("running test: a1+#-? öäüß");
        (bricked_path, keys_path) = retrieve_path(("bricked", "keys"), ".dnk", true)?;
    } else {
        payload = retrieve_payload()?;
        (bricked_path, keys_path) = retrieve_path(("bricked", "keys"), ".dnk", false)?;
    }

    let encrypted_payload = process_encrypt::key_encryption::run(&payload, size_key);

    let buffered_payload = process_encrypt::key_buffering::run(&encrypted_payload);
    let mut binded_payload = bind(&buffered_payload.1, buffered_payload.0[0].key.len() as i64);

    binded_payload.1 = process_encrypt::decentralization::decentralize(binded_payload.1);

    match write_file(
        binded_payload,
        buffered_payload,
        encrypted_payload.0,
        bricked_path,
        keys_path,
    ) {
        Ok(_) => Ok(String::from("Success converting.")),
        Err(err) => panic!("Critical error {} while encrypting", err),
    }
}

fn retrieve_payload() -> anyhow::Result<String> {
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
fn retrieve_path(
    name: (&str, &str),
    format: &str,
    burst: bool,
) -> anyhow::Result<(String, String)> {
    let path;
    if burst {
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
        Ok((name.0.to_string() + format, name.1.to_string() + format))
    } else {
        Ok((
            path.clone() + "/" + name.0 + format,
            path + "/" + name.1 + format,
        ))
    }
}
fn write_file(
    binded_payload: (Vec<BindingKey>, String),
    encrypted_payload: (Vec<BufferKey>, String),
    keys: Vec<EncryptionKey>,
    bricked_path: String,
    keys_path: String,
) -> anyhow::Result<()> {
    let mut bricked_file: File = File::create(&bricked_path)?;
    let mut keys_file: File = File::create(&keys_path)?;
    let mut to_write_keys: String = String::new();

    bricked_file.write(binded_payload.1.as_bytes())?;

    let last_object_0 = encrypted_payload.0[encrypted_payload.0.len() - 1]
        .key
        .clone();
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

    to_write_keys = process_encrypt::decentralization::decentralize(to_write_keys);
    keys_file.write(to_write_keys.as_bytes())?;

    Ok(())
}
