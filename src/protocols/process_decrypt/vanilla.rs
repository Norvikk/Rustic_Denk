use crate::protocols::process_encrypt::key_encryption::EncryptionKey;

pub fn determine_keys(chunk: String) -> anyhow::Result<Vec<EncryptionKey>> {
    let mut keys: Vec<EncryptionKey> = vec![];

    let synapse = "#4m";
    let split_chunk: Vec<&str> = chunk.split(synapse).collect();

    let mut jump = false;
    let mut jump_value = "";
    for n in 0..split_chunk.len() {
        if !jump {
            jump_value = split_chunk[n];
            jump = true;
        } else {
            keys.push(EncryptionKey {
                key: split_chunk[n].to_string(),
                symbol: jump_value.chars().next().unwrap(),
            });
            jump = false;
        }
    }

    Ok(keys)
}

pub fn determine_payload(keys: Vec<EncryptionKey>, brick: &str) -> anyhow::Result<String> {
    let mut payload = String::new();
    let key_size = keys[0].key.len() as i64;
    let iterable = split_string(brick, key_size);

    //println!("{:?}",(iterable.clone(), brick, key_size));

    for bricked in iterable {
        for key in keys.iter() {
            if key.key == bricked {
                payload.push(key.symbol);
            }
        }
    }

    fn split_string(s: &str, chunk_size: i64) -> Vec<String> {
        let mut result = vec![];
        let mut start = 0;
        let mut end = chunk_size as usize;

        while start < s.len() {
            let chunk = s[start..end].to_string();
            result.push(chunk);
            start = end;
            end += chunk_size as usize;
        }

        result
    }

    Ok(payload)
}
