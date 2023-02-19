pub fn options() {
    let options: [&str; 3] = ["Encrypt (save)", "Decrypt (save)", "View Decrypted (temporal)"];

    println!("Choose from the following ->");

    for (i, item) in options.iter().enumerate() {
        println!("[{i}]: {item}", i = i, item = item)
    }
}