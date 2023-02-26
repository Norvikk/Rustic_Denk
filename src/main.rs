pub mod protocols;
pub mod user_interaction;


fn main() -> anyhow::Result<()> {

    // Displays the options the user has to choose
    user_interaction::display::options();

    // Gathers the user input
    let option = user_interaction::requests::get_option()?;

    // Attempts to forward the process to whatever protocol the user chose
    

    match  forward_process(option){
        Ok(value) => println!("Succeeded! -> {}", value),
        Err(err) => panic!("A fatal error caused instability -> {}", err)
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


// TODO fix special symbol indexing 