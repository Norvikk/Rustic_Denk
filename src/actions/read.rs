use std::fs;


use crate::ProcessConfig;

pub fn files(config: &mut ProcessConfig){
    config.read_keys = fs::read_to_string("Keys.dnk").expect("Error occured reading Keys.dnk"); // TODO: add custom path support
    config.read_blur = fs::read_to_string("Brick.dnk").expect("Error occured reading Brick.dnk"); // TODO: add custom path support
}
