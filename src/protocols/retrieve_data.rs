pub struct DataSet {
    user_input: String,
    bricked_user_input: String,
   
}

impl DataSet {
    pub fn new(s1: String, s2: String,) -> Self {
        Self { user_input: s1, bricked_user_input: s2,  }
    }

    pub fn set_user_input(&mut self, new_value: String) {
        self.user_input = new_value;
    }

    pub fn set_bricked_user_input(&mut self, new_value: String) {
        self.bricked_user_input = new_value;
    }


    pub fn get_user_input(&self) -> &String {
        &self.user_input
    }

    pub fn get_bricked_user_input(&self) -> &String {
        &self.bricked_user_input
    }

}