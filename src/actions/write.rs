use std::fs::File;
use std::io::Write;
use crate::ProcessConfig;

pub fn files(config: &ProcessConfig, format: &str) -> anyhow::Result<()>{
    let mut key_file = File::create(format!("Keys{}", format))?;

    for (key, value) in config.process_soft_bundle.iter() {
        let line = format!("{}{}{}{}", key, config.system_synapse, value, config.system_synapse); // FIXME: Make this unreadable
        key_file.write_all(line.as_bytes())?;
    }

    let mut blur_file = File::create(format!("Brick{}", format))?;

    blur_file.write(config.process_blur_payload.as_bytes())?;

    Ok(())
}
