use super::key_encryption::EncryptionKey;
use rand::Rng;

pub fn run(encrypted_payload: &(Vec<EncryptionKey>, String)) -> (Vec<BufferKey>, String)  {
    let mut buffer_payload: (Vec<BufferKey>, String) = (vec![], String::new()) ; 
    let length_key = encrypted_payload.0[1].key.chars().count() as i64 / 2;

    for indexed in &encrypted_payload.0{
       buffer_payload.0.push(BufferKey { key: random_mixed_string(length_key), symbol: indexed.key.clone() })
    }

    let iterable = split_bricked(encrypted_payload.0[0].key.chars().count() as i64, &encrypted_payload.1);
    let mut buffer_result = String::new();

    for index in iterable {
        for index_key in buffer_payload.0.iter(){
            if index_key.symbol == index {
                buffer_result.push_str(&index_key.key)
            }
        }
    }

    buffer_result.push_str("ADCVM235");
    buffer_payload.1 = buffer_result.clone();
    
    
    let temporal_decrypt_keys_size = split_bricked(buffer_payload.0[0].key.chars().count() as i64, &buffer_payload.1);
    let mut temporal_debuffered_entry: Vec<String> = vec![];

    for split_index in temporal_decrypt_keys_size{
        for keys_buffer in buffer_payload.0.iter() {
            if split_index == keys_buffer.key {
                temporal_debuffered_entry.push(keys_buffer.symbol.clone());
            }
        }
    }


    let mut vanilla_result: String = String::new();
    for debuffered_index in temporal_debuffered_entry{
        for keys_vanilla in encrypted_payload.0.iter(){
            if debuffered_index == keys_vanilla.key{
                vanilla_result.push(keys_vanilla.symbol);
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

    println!("{}", vanilla_result);

    return buffer_payload;
}

#[derive(Debug)]
pub struct BufferKey {
    pub key: String,
    pub symbol: String,
}

fn random_mixed_string(length: i64) -> String {
    let mixed: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNO0123456789PQRSTUVWXYZ!#$%&()*+,-./:;<=>?@[]^_`{|}".chars().collect();
    let mut rng = rand::thread_rng();

    let mut result = String::new();
    for _ in 0..length {
        result.push(mixed[rng.gen_range(0..mixed.len())])
    }

    result
}