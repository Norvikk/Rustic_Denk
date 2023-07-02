use std::fs::File;
use std::io::Write;
use std::fs;
use crate::ProcessConfig;

pub fn files(config: &ProcessConfig, format: &str) {
    let mut key_file = File::create(format!("./textcrypt/Keys{}", format)).unwrap();

    for (key, value) in config.process_soft_bundle.iter() {
        let line = format!("{}{}{}{}", key, config.system_synapse, value, config.system_synapse); // FIXME: Make this unreadable
        key_file.write_all(line.as_bytes()).unwrap();
    }

    let mut blur_file = File::create(format!("./textcrypt/Brick{}", format)).unwrap();

    blur_file.write(config.process_created_blur.as_bytes()).unwrap();

}

pub fn flush_dnk() {
    let _ = fs::remove_file("./textcrypt/Brick.dnk");
    let _ = fs::remove_file("./textcrypt/Keys.dnk");
}

pub fn flush_filecrypt() {
    let paths = fs::read_dir("./filecrypt/2_output_encrypt").unwrap();
    
    for path in paths {
        let _ = fs::remove_file(path.unwrap().path());
    }
}