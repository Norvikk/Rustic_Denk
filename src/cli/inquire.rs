use inquire::{InquireError, Select};
use inquire::{Text, validator::{Validation}};



pub fn process(processes: &Vec<String>) -> usize{
    // creates the CLI using crate inquire
    let selection: Result<String, InquireError> =
        Select::new("Select process", processes.clone()).prompt();

    let selection_result = selection.unwrap();

    // returns the index of the users selection
    processes
        .iter()
        .position(|r| r == &selection_result)
        .unwrap()
}



pub fn get_key_data() -> usize {

    let validator = |input: &str| {
        match input.parse::<usize>() {
            Ok(_) => {if input != "0" {Ok(Validation::Valid)} else {Ok(Validation::Invalid("Invalid. It must be a number and above 0".into()))}},
            Err(_) => Ok(Validation::Invalid("Invalid. It must be a number and above 0".into())),
        }
    };

    let status = Text::new("Key size:")
        .with_validator(validator)
        .with_help_message("3 - 256 is recommended")
        .prompt();


    status.unwrap().parse::<usize>().unwrap() 
}

pub fn get_text_data() -> String {
    
    let validate_text = |input: &str| {
        if input.to_string().is_empty() {
            Ok(Validation::Invalid("This cannot be empty!".into()))
        } else {
            Ok(Validation::Valid)
        }
    };

    let status = Text::new("To encrypt:")
    .with_validator(validate_text)
    .with_help_message("Insert what you want to encrypt")
    .prompt();


    status.unwrap().to_string()
}

pub fn get_seed_data() -> String {
    
    let validate_text = |input: &str| {
        if input.to_string().is_empty() {
            Ok(Validation::Invalid("This cannot be empty!".into()))
        } else {
            Ok(Validation::Valid)
        }
    };

    let status = Text::new("Password:")
    .with_validator(validate_text)
    .with_help_message("This password will be used for decrypting the files")
    .prompt();


    status.unwrap().to_string()
}
