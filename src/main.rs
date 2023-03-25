use rustic_denk_algo::protocols;
use rustic_denk_algo::user_interaction;

fn main() -> anyhow::Result<()> {
    user_interaction::display::possible_processes();
    let user_option = user_interaction::requests::request_option()?;
    let mut user_key_size: i64 = 12;
    if user_option == 0 {
        user_key_size = user_interaction::requests::request_key_size()?;
    }

    println!("Succeeded! -> {}", forward_process(user_option, user_key_size)?);


     
    Ok(())
}

pub fn forward_process(option: i8, key_size: i64) -> anyhow::Result<String> {
    match option {
        0 => Ok(protocols::encrypt::run(false, key_size)?),
        1 => Ok(protocols::decrypt::start(false, true)?),
        2 => Ok(protocols::decrypt::start(false, false)?),
        _ => {
            println!("\nInvalid.\n\n");
            user_interaction::display::possible_processes();
            forward_process(
                user_interaction::requests::request_option()?,
                user_interaction::requests::request_key_size()?,
            )
        }
    }
}
