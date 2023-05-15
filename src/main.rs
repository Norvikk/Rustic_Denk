use std::collections::HashMap;
use std::time::Instant;


use denk_algo::actions;
use denk_algo::cli;
use denk_algo::processes;
use denk_algo::utility;

use denk_algo::ProcessConfig;

fn main() -> anyhow::Result<()> {
    cli::display::developer(); // Displays info about the developer and the program

    let mut config: ProcessConfig = ProcessConfig {
        // Need initialization
        system_synapse: utility::generate::random_string(true, true, true, true, 10), // If size != 10 change /processes/justify_keys/assign_synapse/count_of_steps

        user_key_length: 1_000_00,
        user_clear_payload: String::from("hello world"),

        // Need no initialization
        process_soft_bundle: HashMap::new(),
        process_blur_payload: String::new(),
        read_blur: String::new(),
        read_keys: String::new(),
    };

    let processes: Vec<&str> = vec![
        "Encrypt (save)",
        "Decrypt (save)\n",
        "Exit process\n",
        "Benchmark 0 [Low]",
        "Benchmark 1 [Medium]",
        "Benchmark 2 [High]",
        "Benchmark 3 [Absurd]",
    ];


    // request the user process with crate inquire then forwards that index to choose a process
    // forward_process(&mut config, cli::inquire::process(processes)?);

    forward_process(&mut config, 0);

   //  println!("The median is {} ms", benchmark_process(&mut config, 1, 200));
    

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
            processes::encrypt::encrypt(config);
            processes::decrypt::decrypt(config);
        }
        4 => {}
        5 => {}
        _ => {}
    }
     println!("Process {} took {} ms", index, start_time.elapsed().as_millis());

    return start_time.elapsed().as_millis() as usize;
}

fn benchmark_process(config: &mut ProcessConfig, process_index: usize, loops: usize) -> usize {
    let mut timings: usize = 0;

    for _ in 0..loops { timings += forward_process(config, process_index); }

    timings/loops
}