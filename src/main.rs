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
    let processes: Vec<&str> = vec!["Encrypt (save)", "Decrypt (save)\n", "Exit process\n", "Benchmark 0 ", "Reliability test (100 runs)", "Flush Brick.dnk/Keys.dnk files"]; 

    loop {
        cli::display::developer(); 

        let mut config: ProcessConfig = ProcessConfig {
            // Need discrete initialization
            user_key_length: 1000,     // user this up
            user_clear_payload: String::from("Ok I am done. Pushing to git..."), // user this up
            process_chosen_index: cli::inquire::process(processes.clone()), 
    
            // Need no initialization
            system_synapse: utility::generate::random_string(10),
    
            process_soft_bundle: HashMap::new(),
            process_created_blur: String::new(),
    
            read_blur: String::new(),
            read_keys: String::new(),
        };
    
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
        3 => {  config.process_chosen_index = 0; println!("The encryption average is {} ms at 100 loops accuracy", benchmark_process(config, 100)); 
                config.process_chosen_index = 1; println!("The decryption average is {} ms at 100 loops accuracy", benchmark_process(config, 100));
                config.process_chosen_index = 100; }

        4 => processes::reliability::reliability_process(100, true),
        5 => {actions::write::flush_dnk();}
        _ => {}
    }
    
    return start_time.elapsed().as_millis() as usize;
}

fn benchmark_process(config: &mut ProcessConfig, loops: usize) -> usize {
    let mut timings: usize = 0;
    let old_config = config.clone();
    for _ in 0..loops {
        timings += forward_process(config);
        config.reset_to(&old_config)
    }
    timings / loops
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