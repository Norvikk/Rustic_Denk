use std::{collections::HashMap};

pub mod actions;
pub mod cli;
pub mod processes;
pub mod utility;



#[derive(Clone)]
pub struct ProcessConfig {
    // System
    pub system_synapse: String,

    // User
    pub user_key_length: usize,
    pub user_clear_payload: String,

    // Process
    pub process_soft_bundle: HashMap<String, String>,
    pub process_created_blur: String,

    // Read
    pub read_keys: String,
    pub read_blur: String,

}

impl ProcessConfig {
    pub fn flush(&mut self) {
        self.system_synapse.clear();

        self.user_clear_payload.clear();
        self.user_key_length = 0;

        self.process_soft_bundle.clear();
        self.process_created_blur.clear();

        self.read_blur.clear();
        self.read_keys.clear();

    }

    pub fn flush_keys(&mut self) {
        self.read_keys.clear();
    }

    pub fn flush_blur(&mut self) {
        self.read_blur.clear();
    }

    pub fn reset_to(&mut self, default: &ProcessConfig) {
        // System
        self.system_synapse = default.system_synapse.clone();

        // User
        self.user_key_length = default.user_key_length;
        self.user_clear_payload = default.user_clear_payload.clone();

        // Process
        self.process_soft_bundle = default.process_soft_bundle.clone();
        self.process_created_blur = default.process_created_blur.clone();

        // Read
        self.read_keys = default.read_keys.clone();
        self.read_blur = default.read_blur.clone();


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
            process_created_blur: String::new(),
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
