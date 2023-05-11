pub mod cli;
pub mod process_config;

#[cfg(test)]
mod functionality_tests {
    use crate::process_config::{ ProcessConfig};


    #[test]
    fn struct_process_config() {
        let mut process_config_file = ProcessConfig {
            synapse: String::from("synapse_structure"),
            iteration: 190,
        };

        assert_eq!(process_config_file.iteration, 190);
        assert_eq!(process_config_file.synapse, "synapse_structure");

        process_config_file.cli_display();
        process_config_file.flush();

        assert_eq!(process_config_file.iteration, 0);
        assert_eq!(process_config_file.synapse, "");
    }
}