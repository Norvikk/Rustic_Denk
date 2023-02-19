use rustic_denk_algo::{self, user_interaction, protocols::{self, retrieve_data}};
use anyhow::{Result, Error};

fn main() {

    
    
    // Displays the options the user has to choose
    user_interaction::display::options();
   
    // Gathers the user input
    let option = user_interaction::requests::get_option();

    // Attempts to forward the process to whatever protocol the user chose
    forward_process(option)

   
    
}

pub fn forward_process(option: Result<i8, Error>)  {
let stored = retrieve_data::DataSet::new("Hello".to_string(), "world".to_string());
 match option {
     Ok(output) => {
        match output {
            0 => protocols::encrypt::run(),
            1 => protocols::decrypt::run(stored),
            2 => protocols::single_view::run(),
            _ => panic!("No such option index {}", output)
        }
     }
     Err(error) => {panic!("Unexpected input at option selection, error: {}", error)}
    }
}
