use crate::ProcessConfig;

use super::encrypt_source::soft_encrypt::soft_encrypt;

pub fn encrypt(config: &mut ProcessConfig) {
    soft_encrypt(config);
}