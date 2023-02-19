
pub fn get_option() -> anyhow::Result<i8> {
    let result = inquire::Text::new("Select: ").prompt()?.trim().to_owned();

    match result.trim().parse::<i8>() {
        Ok(ok) => Ok(ok),
        Err(e) => panic!("Failed string-to-int8 conversion at: {} ", e), 
    }
}


