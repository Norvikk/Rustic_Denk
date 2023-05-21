use std::collections::HashMap;
use std::time::Instant;
use crossterm::event::{poll, read, Event};
use colored::Colorize;


use denk_algo::actions;
use denk_algo::cli;
use denk_algo::processes;
use denk_algo::utility;

use denk_algo::ProcessConfig;

fn main() -> ! {
    let processes: Vec<String> = vec![
        "Encrypt (save)".bold().to_string(),
        "Decrypt (save)\n".bold().to_string(),
        "Exit process\n".red().bold().to_string(),
        "Reliability test\n".blue().to_string(),
        "Flush Brick.dnk/Keys.dnk files".black().italic().to_string(),
        "Re-enter values".black().italic().to_string(),
    ];

    let mut config: ProcessConfig = ProcessConfig {

        user_clear_payload: cli::inquire::get_text_data(),
        user_key_length: cli::inquire::get_key_data(),
        process_chosen_index: 100, 
        
        system_synapse: utility::generate::random_string(10),

        process_soft_bundle: HashMap::new(),
        process_created_blur: String::new(),

        read_blur: String::new(),
        read_keys: String::new(),
    };

    let copy_config: ProcessConfig = config.clone();

    loop {
        cli::display::developer(); 


        config.reset_to(&copy_config);
        config.process_chosen_index = cli::inquire::process(&processes);
    
        forward_process(&mut config);

        if config.process_chosen_index == 0 { println!("\n{}", "Encryption successful!".bold().green());}
        else if config.process_chosen_index == 1 { println!("{} -> {}", "Decryption successful!".bold().green(), config.user_clear_payload.bold().black()); }
        
        
        press_any_key_to_continue();
        clearscreen::clear().unwrap();
    }
    
}

fn forward_process(config: &mut ProcessConfig) -> usize {
    let start_time = Instant::now();

    match config.process_chosen_index {
        0 => {
            processes::encrypt::encrypt(config);
            actions::write::files(&config, ".dnk");}
        1 => {
            processes::decrypt::decrypt(config);
            
        }
        2 => std::process::exit(-1),
        3 => { processes::reliability::reliability_process(100, true) }

        4 => actions::write::flush_dnk(),
        5 => {clearscreen::clear().unwrap(); main()},
        _ => {}
    }
    
    return start_time.elapsed().as_millis() as usize;
}


fn press_any_key_to_continue() {
    println!("{}", "\n────────────────────────────────".red().bold());
    println!("{}", "   Press enter to continue...   ".bold());
    println!("{}", "────────────────────────────────\n".red().bold());
    
    loop {
        if poll(std::time::Duration::from_secs(0)).unwrap() {
            if let Event::Key(_) = read().unwrap() {
                break;
            }
        }
    }
}