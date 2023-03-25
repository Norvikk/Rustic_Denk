use inquire::{validator::Validation, *};

pub fn request_option() -> anyhow::Result<i8> {
    let key_size_validator = |input: &i8| {
        if *input > 2 {
            Ok(Validation::Invalid("This number is not valid".into()))
        } else {
            Ok(Validation::Valid)
        }
    };

    let user_option = CustomType::<i8>::new("Which option?")
        .with_formatter(&|i| format!("{:.2}", i))
        .with_error_message("Please type a valid number.")
        .with_help_message("Any of the displayed numbers")
        .with_placeholder("0")
        .with_validator(key_size_validator)
        .prompt()?;

    Ok(user_option)
}

pub fn request_key_size() -> anyhow::Result<i64> {
    let maximum_i64_validator = |input: &i64| {
        if *input >= i64::MAX {
            Ok(Validation::Invalid(
                "The number is too big for the supported format i64".into(),
            ))
        } else if *input < 2 {
            Ok(Validation::Invalid("The number is too small".into()))
        } else {
            Ok(Validation::Valid)
        }
    };

    let user_key_size = CustomType::<i64>::new("Keys-Length ")
        .with_formatter(&|i| format!("{:.2}", i))
        .with_error_message("The number is either too small or too big.")
        .with_help_message("Iteraion keys: (lower -> faster) : (512 is high)")
        .with_placeholder("[low: 4] [med: 64] [high: 128 - 512")
        .with_validator(maximum_i64_validator)
        .prompt()?;

    Ok(user_key_size)
}
