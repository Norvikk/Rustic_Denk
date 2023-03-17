use inquire::validator::Validation;
use std::fs::File;
use std::io::prelude::*;

use super::process_encrypt;
use super::process_encrypt::binding::{bind, BindingKey};
use super::process_encrypt::key_buffering::BufferKey;
use super::process_encrypt::key_encryption::EncryptionKey;

use rustic_denk_algo::synapse_host;

pub fn run() -> anyhow::Result<()> {
    let synapse_design = "?s4";
    let payload: String = retrieve_payload()?;
    let encrypted_bundle: (Vec<EncryptionKey>, String) =
        process_encrypt::key_encryption::run(&payload);

    let (bricked_path, keys_path): (String, String) = retrieve_path(("bricked", "keys"), ".dnk")?;
    let buffered_bundle: (Vec<BufferKey>, String) =
        process_encrypt::key_buffering::run(&encrypted_bundle);

    let mut binded_bundle: (Vec<BindingKey>, String) =
        bind(&buffered_bundle.1, buffered_bundle.0[0].key.len() as i64);

    binded_bundle.1 = process_encrypt::decentralization::decentralize(binded_bundle.1);

    write_file(
        binded_bundle,
        buffered_bundle,
        encrypted_bundle.0,
        bricked_path,
        keys_path,
        synapse_design
    )?;

    Ok(())
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
fn retrieve_path(name: (&str, &str), format: &str) -> anyhow::Result<(String, String)> {
    let path = inquire::Text::new("File Directory: ")
        .with_help_message(
            "Leaving this empty writes the output to this folder where the app is located",
        )
        .with_placeholder("Sample: C:/Users/Administrator/Downloads")
        .prompt()?
        .trim()
        .to_owned();

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
    binded_bundle: (Vec<BindingKey>, String),
    buffer_bundle: (Vec<BufferKey>, String),
    encryption_keys: Vec<EncryptionKey>,
    bricked_path: String,
    keys_path: String,  
    synapse_design: &str
) -> anyhow::Result<()> {
    let mut bricked_file: File = File::create(&bricked_path)?;
    let mut keys_file: File = File::create(&keys_path)?;
    let mut to_write_keys: String = String::new();

    bricked_file.write(binded_bundle.1.as_bytes())?;

    let last_object_0 = buffer_bundle.0[buffer_bundle.0.len() - 1].key.clone();
    for object in buffer_bundle.0.iter() {
        let to_write: String;
    
        if object.key != last_object_0 {
            to_write = format!("{}{}{}", object.symbol.to_string(), synapse_design, object.key);
        } else {
            to_write = format!("{}{}{}", object.symbol.to_string(), synapse_design, object.key);
        }
    
        to_write_keys.push_str(&to_write);
    }

    to_write_keys.push_str("BUFFER");

    for object in encryption_keys.iter() {
        let to_write = format!("{}{}{}", object.symbol.to_string(), synapse_design, object.key);
    
        to_write_keys.push_str(&to_write);
    }

    to_write_keys.push_str("BINDING");

    let last_object_1 = binded_bundle.0[binded_bundle.0.len() - 1].key.clone();
    for object in binded_bundle.0.iter() {
        let to_write: String;
    
        if object.key != last_object_1 {
            to_write = format!(
                "{}{}{}{}{}",
                object.symbol.0.to_string(),
                synapse_design,
                object.symbol.1.to_string(),
                synapse_design,
                object.key
            );
        } else {
            to_write = format!(
                "{}{}{}{}",
                object.symbol.0.to_string(),
                synapse_design,
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
