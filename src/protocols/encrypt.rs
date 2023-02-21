use core::panic;
use inquire::validator::Validation;
use std::fs::File;
use std::io::prelude::*;

use super::process_encrypt;

pub fn run() -> anyhow::Result<String> {
    let mut bricked_file: File;
    let mut keys_file: File;
    println!("0");

    let payload = retrieve_payload()?;

    let bricked_path = retrieve_path("bricked")?;

    let keys_path = retrieve_path("keys")?;

    let encrypted_payload = process_encrypt::key_encryption::run(&payload);

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

    Ok(encrypted_payload.1)
}

fn retrieve_payload() -> anyhow::Result<String> {
    let validator = |input: &str| {
        if input.chars().count() < 1 {
            Ok(Validation::Invalid("This should not be left empty.".into()))
        } else {
            Ok(Validation::Valid)
        }
    };

    let user_payload = inquire::Text::new("What should be encrypted?")
        .with_validator(validator)
        .prompt();

    match user_payload {
        Ok(value) => Ok(value),
        Err(err) => panic!("{} was empty. STRING OF SIZE NULL", err),
    }
}

fn retrieve_path(name: &str) -> anyhow::Result<String> {
    let path = inquire::Text::new("Where should the text file be written? (leave empty = origin)")
        .prompt()?
        .trim()
        .to_owned();

    let format: &str = ".txt";

    if path.is_empty() {
        Ok(name.to_string() + format)
    } else {
        Ok(path + "/" + name + format)
    }
}
