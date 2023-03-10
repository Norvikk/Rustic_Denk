use crate::protocols::process_encrypt::binding::BindingKey;

pub fn unbind(read_bricked: &str, bricked_content: &str) -> String {
    let mut result = String::new();
    let synapse = "#4m";
    let split_keys = read_bricked.split(synapse);

    let mut binding_keys: Vec<BindingKey> = vec![];

    let mut entry_looper: i8 = 0;
    let mut entry1: String = String::new();
    let mut entry2: String = String::new();

    for split in split_keys {
        if entry_looper == 0 {
            entry1 = split.to_string();
            entry_looper += 1;
        } else if entry_looper == 1 {
            if split.to_string() == "FILLER" {
                entry2 = "".to_string();
            } else {
                entry2 = split.to_string();
            }

            entry_looper += 1;
        } else {
            binding_keys.push(BindingKey {
                symbol: (entry1.clone(), entry2.clone()),
                key: (split.to_string()),
            });

            entry_looper = 0;
        }
    }

    let split_read_bricked = combine_strings(split_string(
        bricked_content,
        binding_keys[0].symbol.0.len() as i64,
    ));
    for index in split_read_bricked.iter() {
        for key in binding_keys.iter() {
            if index == &key.key {
                let to_push = format!("{}{}", key.symbol.0, key.symbol.1);
                result.push_str(&to_push);
            }
        }
    }

    result
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

fn combine_strings(v: Vec<String>) -> Vec<String> {
    let mut result = Vec::new();
    let mut i = 0;
    while i < v.len() {
        if i + 1 < v.len() {
            let combined_string = v[i].clone() + &v[i + 1];
            result.push(combined_string);
        } else {
            result.push(v[i].clone());
        }
        i += 2;
    }
    result
}
