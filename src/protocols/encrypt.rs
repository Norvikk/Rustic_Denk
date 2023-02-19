use inquire::validator::Validation;
use std::fs::File;
use std::io::prelude::*;

pub fn run() -> anyhow::Result<String> {
    let mut file: File;
    let payload = retrieve_payload()?;
    let path = retrieve_path("bricked")?;

    file = File::create(path.clone())?;
    file.write(payload.as_bytes())?;

    Ok("This".to_string())
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
    let path = inquire::Text::new("Where should the files be written? (leave empty = origin)")
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
