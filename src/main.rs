pub mod user_interaction;
pub mod protocols;

fn main() -> anyhow::Result<()> {
        
    // Displays the options the user has to choose
    user_interaction::display::options();
   
    // Gathers the user input
    // let option = user_interaction::requests::get_option()?;
    let option  = 0;

    // Attempts to forward the process to whatever protocol the user chose
    let resultof = forward_process(option)?;

    println!("{}", resultof);

    Ok(())
}

pub fn forward_process(option: i8) -> anyhow::Result<String>  {
        match option {
            0 => Ok(protocols::encrypt::run()?),
            1 =>  Ok(protocols::decrypt::run()),
            2 =>  Ok(protocols::single_view::run()),
            _ => panic!("No such option index {}", option)
        }
}
