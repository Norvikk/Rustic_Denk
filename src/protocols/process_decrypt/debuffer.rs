use anyhow::Ok;

use crate::protocols::process_encrypt::key_buffering::BufferKey;

pub fn determine_keys(chunk: String) -> anyhow::Result<Vec<BufferKey>> {
    let mut keys: Vec<BufferKey> = vec![];

    let split_chunk: Vec<&str> = chunk.split("?s§0-a").collect();
    

    let mut jump = false;
    let mut jump_value = "";
    for n in 0..split_chunk.len(){
        if !jump {
            jump_value = split_chunk[n];
            jump = true;
        } else {
            keys.push(BufferKey {
                key: split_chunk[n].to_string(),
                symbol: jump_value.to_string(),
            });
            jump = false;
        }
    }

    Ok(keys)
}


pub fn determine_payload(keys: Vec<BufferKey>, brick: &str) -> anyhow::Result<String> {
    let mut payload = String::new();
    let key_size = keys[0].key.len() as i64;
    let iterable = split_bricked(key_size, brick);

    for bricked in iterable {
        for key in keys.iter() {
            if key.key == bricked {
                payload.push_str(&key.symbol);
            }
        }
    }

    fn split_bricked(keysize: i64, brick: &str) -> Vec<String> {
        let mut slices: Vec<String> = vec![];

        let mut looper = 0;
        let mut result = String::new();
        for char in brick.chars() {
            if looper == keysize {
                slices.push(result);
                result = char.to_string();
                looper = 1;
            } else {
                looper += 1;
                result.push(char);
            }
        }

        return slices;
    }

payload.push_str("ADCVM235");
    Ok(payload)
}