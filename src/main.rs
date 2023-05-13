use std::collections::HashMap;

use denk_algo::cli;
use denk_algo::processes;
use denk_algo::utility;
use denk_algo::actions;

use denk_algo::ProcessConfig;

fn main() -> anyhow::Result<()> {
    cli::display::developer(); // Displays info about the developer and the program
    let is_development: bool = true;

    let mut config: ProcessConfig = ProcessConfig {
        //system_synapse: utility::generate::random_string(true, true, true, true, 8),
        system_synapse: String::from("1234567890"),
        user_key_length: 4,
        //user_clear_payload: utility::generate::random_string(true, true, true, true, 8),
        user_clear_payload: String::from("qwerasdfyxcv"),
        //user_clear_payload: String::from("aabcd//"),
        process_soft_bundle: HashMap::new(),
        process_blur_payload: String::new(),
        read_blur: String::new(),
        read_keys: String::new(),
    };

    let mut processes: Vec<&str> = vec!["Encrypt (save)", "Decrypt (save)\n"];
    if is_development {
        processes.push("Exit process\n");
        processes.push("Benchmark 0 [Absurd]");
        processes.push("Benchmark 1 [High]");
        processes.push("Benchmark 2 [Medium]");
        processes.push("Benchmark 3 [Low]");
    }

    // request the user process with crate inquire then forwards that index to choose a process
    forward_process(
        &mut config,
        cli::inquire::process(processes)?,
        is_development,
    );

   



    Ok(())
}

fn forward_process(config: &mut ProcessConfig, index: usize, is_developing: bool) {
    if is_developing {
        match index {
            0 => {processes::encrypt::encrypt(config);  actions::write::files(&config, ".dnk").unwrap();},
            1 => {processes::decrypt::decrypt(config)}
            2 => std::process::exit(-1),
            3 => {}
            4 => {}
            5 => {}
            _ => {}
        }
    } else {
        match index {
            0 => {}
            1 => {}
            2 => {}
            3 => {}
            _ => {}
        }
    }
}
