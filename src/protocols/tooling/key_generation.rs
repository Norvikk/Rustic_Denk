static mut LIST: Vec<String> = Vec::new();

use rand::Rng;

pub fn unique_key(length: i64) -> String {
    unsafe {
        let mixed: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNO0123456789PQRSTUVWXYZ!#$%&()*+,-./:;<=>?@[]^_`{|}".chars().collect();
        let mut rng = rand::thread_rng();
        let retry_tolerance: i32 = 900;
        let mut result = String::new();
        for _ in 0..length {
            result.push(mixed[rng.gen_range(0..mixed.len())])
        }

        if LIST.contains(&result) {
            println!("DUPLICATE DETECTED");

           
            for i in 0..retry_tolerance {
                result = String::new();
                for _ in 0..length {
                    result.push(mixed[rng.gen_range(0..mixed.len())])
                }

                if !LIST.contains(&result) {
                    LIST.push(result.clone());
                    break;
                }

                if i == retry_tolerance{
                    panic!("No more possible keys for this keysize. Increase the keysize or decrease the unique characters.")
                }
            }
            
            
        } else {
            LIST.push(result.clone())
        }

        result
    }
}
