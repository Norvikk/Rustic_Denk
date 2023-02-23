use core::panic;
use inquire::validator::Validation;
use std::fs::File;
use std::io::prelude::*;


use super::process_encrypt;
use super::process_encrypt::key_encryption::EncryptionKey;

pub fn run() -> anyhow::Result<String> {
    

    let payload = retrieve_payload()?;
    let bricked_path;
    let keys_path;

    match retrieve_path(("bricked", "keys"), ".dnk") {
        Ok(value) => {bricked_path = value.0; keys_path = value.1},
        Err(err) => panic!("{err}") 
    }

    let encrypted_payload = process_encrypt::key_encryption::run(&payload);

    

    match write_file(encrypted_payload, bricked_path, keys_path) {
        Ok(_) => Ok("Successfully converted".to_string()),
        Err(err) => panic!("Critical error {} while encrypting", err)
    }

    
}

fn retrieve_payload() -> anyhow::Result<String> {
    let validator = |input: &str| {
        if input.chars().count() < 1 {
            Ok(Validation::Invalid("This should not be left empty.".into()))
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
        .with_help_message("Leaving this empty writes the output to this folder where the app is located")
        .with_placeholder("Sample: C:/Users/Administrator/Downloads")
        .prompt()?
        .trim()
        .to_owned();

    

    if path.is_empty() {
        Ok((name.0.to_string() + format, name.1.to_string() + format))
    } else {
        Ok((path.clone() + "/" + name.0 + format, path + "/" + name.1 + format))
    }
}

fn write_file(encrypted_payload: (Vec<EncryptionKey>, String), bricked_path: String, keys_path: String) -> anyhow::Result<()>{

    let mut bricked_file: File;
    let mut keys_file: File;
    bricked_file = File::create(bricked_path.clone())?;
    keys_file = File::create(keys_path.clone())?;

    bricked_file.write(encrypted_payload.1.as_bytes())?;

    for object in encrypted_payload.0.iter() {
        let to_write = format!("{}?s§0-a{}?s§0-a", object.symbol.to_string(), object.key);

        let handle = keys_file.write(to_write.as_bytes());

        match handle {
            Ok(_) => (),
            Err(err) => panic!("ERROR CONVERTING TO FILE: {} ", err),
        }
    }

    Ok(())
}