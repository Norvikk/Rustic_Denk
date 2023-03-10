use rand::Rng;

pub fn bind(bricked: String, keysize: i64) -> (Vec<BindingKey>, String) {
    let mut binded_keys: Vec<BindingKey> = vec![];

    let mut split_bricked = split_string(&bricked, keysize);

    if split_bricked.len() % 2 != 0 {
        split_bricked.push("FILLER".to_string());
    }

    let mut absolute_binded = String::new();

    let mut looper: bool = false;
    let mut cache_looper: String = String::new();
    for bricked in split_bricked{
        if !looper {
            cache_looper = bricked;
            looper =true;
        } else {
            let entry = random_mixed_string(keysize*2);
            binded_keys.push(BindingKey { symbol: (cache_looper.clone(), bricked), key: (entry.clone()) });
            absolute_binded.push_str(&entry);
            looper = false
        }
    }


    (binded_keys, absolute_binded)
}

fn split_string(s: &str, chunk_size: i64) -> Vec<String> {
    let mut result = vec![];
    let mut start = 0;
    let mut end = chunk_size as usize;
    
    while start < s.len()  {
        
        let chunk = s[start..end].to_string();
        result.push(chunk);
        start = end;
        end += chunk_size as usize;
    }

    
    
    result
}

#[derive(Debug)]
pub struct BindingKey {
    pub symbol: (String, String),
    pub key: String,
}

