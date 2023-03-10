use inquire::{ui::RenderConfig, *, };
use inquire::validator::Validation;

pub fn get_option() -> anyhow::Result<i8> {

    let key: CustomType<u8> = CustomType {
        message: "Option: ",
        help_message: Some("Any of the numbers"),
        error_message: "Please type a valid number".into(),
        default: Some(0),
        placeholder: Some("0"),
        formatter: &|i| format!("{i}"),
        default_value_formatter: &|i| format!("{i}"),
        render_config: RenderConfig::default(),
        parser: &|i| match i.parse::<u8>() {
            Ok(i) => Ok(i),
            Err(_) => Err(()),
        },
        validators: vec![],
    };

    let validator = |input: &str| {
        if input.chars().count() < 1 {
            Ok(Validation::Invalid("This should not be left empty.".into()))
        } else {
            Ok(Validation::Valid)
        }
    };

    
    let key = key.prompt()? as i8;
    Ok(key)
}


