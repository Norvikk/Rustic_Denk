use std::collections::HashMap;
use std::time::Instant;
use crossterm::event::{poll, read, Event};
use colored::Colorize;

use denk_algo::actions;
use denk_algo::cli;
use denk_algo::processes;
use denk_algo::utility;

use denk_algo::ProcessConfig;

fn main() {

    // processes::reliability::reliability_process(10, false);
    
    
    // Stores the display processes. The actual logic behind choosing them is hard-coded
    let processes: Vec<&str> = vec!["Encrypt (save)", "Decrypt (save)\n", "Exit process\n", "Benchmark 0 ", "Reliability test (100 runs)", "Flush Brick.dnk/Keys.dnk"]; 

    loop {
        cli::display::developer(); // Displays info about the developer and the program

        // Creates a new instance of ProcessConfig. This is the root Config which will be used by all processes
        let mut config: ProcessConfig = ProcessConfig {
            // Need discrete initialization
            user_key_length: 1_000, // user this up
            user_clear_payload: String::from("ur mum"), // user this up
    
            // Need no initialization
            system_synapse: utility::generate::random_string(10),
    
            process_soft_bundle: HashMap::new(),
            process_created_blur: String::new(),
    
            read_blur: String::new(),
            read_keys: String::new(),
        };
    
        // request the user process with crate inquire then forwards that index to choose a process
        forward_process(&mut config, cli::inquire::process(processes.clone()));

        if !config.user_clear_payload.is_empty() {
            println!("The processed result is: {}", config.user_clear_payload.bold().black());
        }
        
        
        press_any_key_to_continue();
        clearscreen::clear().unwrap();
    }
    
    
    
}

fn forward_process(config: &mut ProcessConfig, index: usize) -> usize {
    let start_time = Instant::now();
    match index {
        0 => {
            processes::encrypt::encrypt(config);
            actions::write::files(&config, ".dnk");
            
        }
        1 => {
            processes::decrypt::decrypt(config);
            
        }
        2 => std::process::exit(-1),
        3 => {
            println!(
                "The encryption average is {} ms at 100 loops accuracy",
                benchmark_process(config, 0, 100)
            );
            println!(
                "The decryption average is {} ms at 100 loops accuracy",
                benchmark_process(config, 1, 100)
            );
        }
        4 => processes::reliability::reliability_process(100, true),
        5 => {actions::write::flush_dnk();}
        _ => {}
    }
    
    return start_time.elapsed().as_millis() as usize;
}

fn benchmark_process(config: &mut ProcessConfig, process_index: usize, loops: usize) -> usize {
    let mut timings: usize = 0;
    let old_config = config.clone();
    for _ in 0..loops {
        timings += forward_process(config, process_index);
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