use rustic_denk_algo::user_interaction;
use rustic_denk_algo::protocols;

fn main() -> anyhow::Result<()> {

    user_interaction::display::options();
    let option = user_interaction::requests::get_option()?;
    let mut key_size: i64 = 12;
    if option != 1 {
        key_size = user_interaction::requests::get_key_size()?;
    }

    match forward_process(option, key_size) {
        Ok(value) => println!("Succeeded! -> {}", value),
        Err(err) => panic!("A fatal error caused instability -> {}", err),
    }

    Ok(())
}

pub fn forward_process(option: i8, key_size: i64) -> anyhow::Result<String> {
    match option {
        0 => Ok(protocols::encrypt::run(false, key_size)?),
        1 => Ok(protocols::decrypt::run(false)?),
        _ => panic!("No such option index {}", option),
    }
}