pub struct ProcessConfig {
    pub synapse: String,
    pub iteration: usize,
}

impl ProcessConfig {
    pub fn flush(&mut self) {
        self.synapse.clear();
        self.iteration = 0;
    }

    pub fn cli_display(&self) {
        println!("[SYNAPSE] {}", self.synapse);
        println!("[ITERATION] {}", self.iteration);
    }
}