use std::fs;


use crate::ProcessConfig;

pub fn files(config: &mut ProcessConfig){
    config.read_keys = fs::read_to_string("./textcrypt/Keys.dnk").expect("Error occured reading Keys.dnk"); 
    config.read_blur = fs::read_to_string("./textcrypt/Brick.dnk").expect("Error occured reading Brick.dnk"); 
}
