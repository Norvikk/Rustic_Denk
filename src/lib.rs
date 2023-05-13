use std::collections::HashMap;

pub mod cli;
pub mod processes;
pub mod utility;
pub mod actions;

pub struct ProcessConfig {
    // System
    pub system_synapse: String,

    // User
    pub user_key_length: usize,
    pub user_clear_payload: String,

    // Process
    pub process_soft_bundle: HashMap<String, String>,
    pub process_blur_payload: String,

    // Read
    pub read_keys: String,
    pub read_blur: String
}



impl ProcessConfig {
    pub fn flush(&mut self) {
        self.system_synapse.clear();
        self.user_key_length = 0;
    }

    pub fn cli_display(&self) {
        let empty_placeholder = "-NULL-";
        if self.system_synapse.is_empty() {
            println!("[SYNAPSE] {}", empty_placeholder);
        } else {
            println!("[SYNAPSE] {}", self.system_synapse);
        }
        println!("[ITERATION] {}", self.user_key_length);
    }
}

#[cfg(test)]
mod functionality_tests {
    use std::collections::HashMap;

    use crate::ProcessConfig;


    #[test]
    fn struct_process_config() {
        let mut process_config_file = ProcessConfig {
            system_synapse: String::from("synapse_structure"),
            user_key_length: 190,
            process_blur_payload: String::new(),
            user_clear_payload: String::from("asdfg"),
            process_soft_bundle: HashMap::new(),
            read_blur: String::new(),
            read_keys: String::new(),
        };

        assert_eq!(process_config_file.user_key_length, 180);
        assert_eq!(process_config_file.system_synapse, "synapse_structure");

        process_config_file.cli_display();
        process_config_file.flush();

        assert_eq!(process_config_file.user_key_length, 0);
        assert_eq!(process_config_file.system_synapse, "");
    }
}