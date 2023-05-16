use std::collections::HashMap;
use std::time::Instant;
use std::process;

use denk_algo::actions;
use denk_algo::cli;
use denk_algo::processes;
use denk_algo::utility;

use denk_algo::ProcessConfig;

fn main() -> anyhow::Result<()> {
    let processes: Vec<&str> = vec![
    "Encrypt (save)",
    "Decrypt (save)\n",
    "Exit process\n",
    "Benchmark 0 [Low]",
    "Benchmark 1 [Medium]",
    "Benchmark 2 [High]",
    "Benchmark 3 [Absurd]",
];

let mut config: ProcessConfig = ProcessConfig {
    // Need initialization
    user_key_length: 1_000_00,
    user_clear_payload: String::from("hello world"),

    // Need no initialization

    system_synapse: utility::generate::random_string(10), 

    process_soft_bundle: HashMap::new(),
    process_created_blur: String::new(),

    read_blur: String::new(),
    read_keys: String::new(),
};


    cli::display::developer();

    config.process_created_blur.reserve(config.user_key_length);


    // request the user process with crate inquire then forwards that index to choose a process
    forward_process(&mut config, cli::inquire::process(processes)?); 
    println!("The decrypted result: {}", config.user_clear_payload);

     

    Ok(())
}

fn forward_process(config: &mut ProcessConfig, index: usize) -> usize {
    let start_time = Instant::now();
    match index {
        0 => {
            processes::encrypt::encrypt(config);
            actions::write::files(&config, ".dnk").unwrap(); 
        }
        1 => {processes::decrypt::decrypt(config);}
        2 => std::process::exit(-1),
        3 => {
            println!("The encryption average is {} ms at 100 loops accuracy", benchmark_process(config, 0, 100));
            println!("The decryption average is {} ms at 100 loops accuracy", benchmark_process(config, 1, 100));
        }
        4 => {}
        5 => {}
        _ => {}
    }
    return start_time.elapsed().as_millis() as usize;
}



fn benchmark_process(config: &mut ProcessConfig, process_index: usize, loops: usize) -> usize {
    let mut timings: usize = 0; let old_config = config.clone();
    for _ in 0..loops { timings += forward_process(config, process_index); config.reset_to(&old_config)}
    timings/loops
}