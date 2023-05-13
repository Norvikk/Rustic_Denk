use crate::ProcessConfig;
use crate::actions::read;


pub fn determine_keys(config: &mut ProcessConfig) {
    read::files(config);
    assign_synapse(config);
    justify_soft(config);

    

}

fn justify_soft(config: &mut ProcessConfig) {
    let split_entries: Vec<&str> = config.read_keys.split(&config.system_synapse).collect();

    let mut skipper = false;
    let mut skipper_value = "empty";

    for index in 0..split_entries.len() {
        if !skipper{
            skipper = true;
            skipper_value = split_entries[index];
        } else {
            config.process_soft_bundle.insert(split_entries[index].to_string(), skipper_value.to_string());
        }
    }
    
    
}

fn assign_synapse(config: &mut ProcessConfig) {
    let count_of_steps = 10;

    config.system_synapse = config.read_keys[1..count_of_steps+1].to_string();
}