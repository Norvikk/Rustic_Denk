use denk_algo::cli;

fn main() -> anyhow::Result<()> {
    
    let mut processes: Vec<&str> = vec!["Encrypt (save)", "Decrypt (save)\n"];
    let is_developing: bool = true; 

    if is_developing {
        processes.push("Exit process\n");
        processes.push("Benchmark 0 [Absurd]");
        processes.push("Benchmark 1 [High]");
        processes.push("Benchmark 2 [Medium]");
        processes.push("Benchmark 3 [Low]");
    }

    cli::display::developer(); // Displays info about the developer and the program

    // request the user process with crate inquire then forwards that index to choose a process
    forward_process(cli::inquire::process(processes)?, is_developing);
    Ok(())
}

fn forward_process(index: usize, is_developing: bool) {
    if is_developing {
        match index {
            0 => std::process::exit(-1),
            1 => {}
            2 => {}
            3 => {}
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

