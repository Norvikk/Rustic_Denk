pub mod protocols;
pub mod user_interaction;

#[cfg(test)]
mod burst_functionality {

    use super::*;

    use std::fs;

    #[test]
    fn encrypt_then_decrypt() -> anyhow::Result<()> {
        fn cleanup_files() -> anyhow::Result<()> {
            let folder_path = "target"; // replace with actual folder path

            // delete "bricked.dnk" file
            let bricked_file_path = format!("{}/{}", folder_path, "bricked.dnk");
            fs::remove_file(&bricked_file_path)?;

            // delete "keys.dnk" file
            let keys_file_path = format!("{}/{}", folder_path, "keys.dnk");
            fs::remove_file(&keys_file_path)?;

            Ok(())
        }
        match forward_process(0, 12) {
            Ok(value) => assert_eq!("Success converting.", value),
            Err(err) => panic!("FATAL ERROR ENCRYPTING -> {}", err),
        }
        match forward_process(1, 12) {
            Ok(value) => assert_eq!("running test: a1+#-? öäüß", value),
            Err(err) => panic!("FATAL ERROR DECRYPTING -> {}", err),
        }

        cleanup_files()?;

        Ok(())
    }


    
}

pub fn forward_process(option: i8, key_size: i64) -> anyhow::Result<String> {
    match option {
        0 => Ok(protocols::encrypt::run(true, key_size)?),
        1 => Ok(protocols::decrypt::start(true, true)?),
        _ => panic!("No such option index {}", option),
    }
}
