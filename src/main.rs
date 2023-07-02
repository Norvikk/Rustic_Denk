use std::collections::HashMap;
use std::time::Instant;
use crossterm::event::{poll, read, Event};
use colored::Colorize;

use std::fs;
use denk_algo::actions;
use denk_algo::cli;
use denk_algo::direct_retrieval::direct_decrypt;
use denk_algo::direct_retrieval::direct_encrypt;
use denk_algo::processes;
use denk_algo::processes::filecrypt::decrypt_with_seed;
use denk_algo::processes::filecrypt::encrypt_with_seed;
use denk_algo::processes::filecrypt::string_to_u64;
use denk_algo::utility;

use denk_algo::ProcessConfig; 

fn main() -> ! {
    create_folder_structure();
    processes::reliability::reliability_process(25, false);
    


        // Usage of API

    // let tuple = direct_encrypt("Hello API World".to_string(), 12);
    // let decrypt = direct_decrypt(tuple.0, tuple.1);
    // println!("{}", decrypt);
    

    let processes: Vec<String> = vec![
        "Encrypt Text (stored)".bold().to_string(),
        "Decrypt Text (stored)".bold().to_string(),
        "Flush Brick.dnk/Keys.dnk files\n".italic().to_string(),
        "Encrypt files (stored)".bold().to_string(),
        "Decrypt files (stored)".bold().to_string(),
        "Flush Filecrypt Decrypt Folder\n".italic().to_string(),
        "Exit process\n".red().bold().to_string(),
    ];

    let mut config: ProcessConfig = ProcessConfig {

        user_clear_payload: String::new(),
        user_key_length: 1, // cannot be 0 now else it breaks decrypt.rs
        process_chosen_index: 100, 
        
        system_synapse: utility::generate::random_string(4),

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
            config.user_clear_payload = cli::inquire::get_text_data();
            config.user_key_length = cli::inquire::get_key_data();
            processes::softcrypt::encrypt(config);
            actions::write::files(&config, ".dnk");
        }
        1 => {
            processes::softcrypt::decrypt(config);
        }
        2 => {
            actions::write::flush_dnk();
        }
        3 => {
            encrypt_with_seed(string_to_u64(&cli::inquire::get_seed_data()));
            println!("Done encrypting");
        }
        4 => {
            decrypt_with_seed(string_to_u64(&cli::inquire::get_seed_data()));
            println!("{} {}",
                "Done decrypting.".bold().green(),
                " (if the password was mistyped or wrong, the files will be unreadable)".to_ascii_uppercase().bold().red()
            );
        }
        5 => {
            actions::write::flush_filecrypt();
        }
        6 => {
            std::process::exit(-1);
        }
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

fn create_folder_structure() {
    let root_folder = "filecrypt";
    let subfolder_names = ["1_input_encrypt", "2_output_encrypt", "3_decrypted"];

    let textcrypt_folder = "textcrypt";

    if let Err(e) = fs::create_dir(textcrypt_folder) {
        if e.kind() != std::io::ErrorKind::AlreadyExists {
            eprintln!("Failed to create textcrypt folder '{}': {}", root_folder, e);
        }
    }

    if let Err(e) = fs::create_dir(root_folder) {
        if e.kind() != std::io::ErrorKind::AlreadyExists {
            eprintln!("Failed to create root folder '{}': {}", root_folder, e);
        }
    }

    

    for subfolder_name in &subfolder_names {
        let folder_path = format!("{}/{}", root_folder, subfolder_name);
        if let Err(e) = fs::create_dir(&folder_path) {
            if e.kind() != std::io::ErrorKind::AlreadyExists {
                eprintln!("Failed to create subfolder '{}': {}", folder_path, e);
            }
        }
    }
}