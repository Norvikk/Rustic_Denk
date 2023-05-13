use crate::ProcessConfig;

use super::{decrypt_source::soft_decrypt::soft_decrypt, justify_keys::{self, determine_keys}};


pub fn decrypt(config: &mut ProcessConfig) {
    determine_keys(config);
    soft_decrypt(config);
}