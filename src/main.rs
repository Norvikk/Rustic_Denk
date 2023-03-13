pub mod protocols;
pub mod user_interaction;

fn main() -> anyhow::Result<()> {

    

    user_interaction::display::options();
    let option = user_interaction::requests::get_option()?;

    match forward_process(option) {
        Ok(value) => println!("Succeeded! -> {}", value),
        Err(err) => panic!("A fatal error caused instability -> {}", err),
    }

    Ok(())
}

pub fn forward_process(option: i8) -> anyhow::Result<String> {
    match option {
        0 => Ok(protocols::encrypt::run()?),
        1 => Ok(protocols::decrypt::run()?),
        _ => panic!("No such option index {}", option),
    }
}