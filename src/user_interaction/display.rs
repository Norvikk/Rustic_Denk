pub fn options() {
    let options: [&str; 2] = ["Encrypt (save)", "Decrypt (save)"];

    println!("Choose from the following ->");

    for (i, item) in options.iter().enumerate() {
        println!("[{i}]: {item}", i = i, item = item)
    }
}
