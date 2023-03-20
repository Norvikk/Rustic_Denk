use rustic_denk_algo::protocols;
use rustic_denk_algo::user_interaction;

fn main() -> anyhow::Result<()> {
    user_interaction::display::options();
    let option = user_interaction::requests::get_option()?;
    let mut key_size: i64 = 12;
    if option != 1 {
        key_size = user_interaction::requests::get_key_size()?;
    }

    println!("Succeeded! -> {}", forward_process(option, key_size)?);

    Ok(())
}

pub fn forward_process(option: i8, key_size: i64) -> anyhow::Result<String> {
    match option {
        0 => Ok(protocols::encrypt::run(false, key_size)?),
        1 => Ok(protocols::decrypt::run(false)?),
        _ => {
            println!("\nInvalid.\n\n");
            user_interaction::display::options();
            forward_process(
                user_interaction::requests::get_option()?,
                user_interaction::requests::get_key_size()?,
            )
        }
    }
}
