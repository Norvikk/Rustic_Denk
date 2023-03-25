pub fn possible_processes() {
    let hard_coded_options: [&str; 3] = ["Encrypt (save)", "Decrypt (save)", "Decrypt without decentralization (save)"];

    println!("Choose from the following ->");

    for (i, item) in hard_coded_options.iter().enumerate() {
        println!("[{i}]: {item}")
    }
}
