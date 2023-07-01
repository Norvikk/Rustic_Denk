use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Read;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use colored::Colorize;


pub fn string_to_u64(input: &str) -> u64 {
    let mut hash: u64 = 0;

    for c in input.chars() {
        hash = (hash.wrapping_mul(31)).wrapping_add(c as u64);
    }

    hash
}

pub fn encrypt_with_seed(seed: u64) {
    let keys = justify_encryption_keys(seed);
    let paths = fs::read_dir("./filecrypt/1_input_encrypt").unwrap();

    for path in paths {
        println!("Encrypting {}", path.as_ref().unwrap().path().display().to_string().bold().green());
        let mut processed_bytes: Vec<u8> = Vec::with_capacity(path.as_ref().unwrap().metadata().unwrap().len() as usize);
        let mut unsafe_reader = File::open(path.as_ref().unwrap().path()).expect("File failed to list");
        let mut unsafe_bytes = Vec::with_capacity(path.as_ref().unwrap().metadata().unwrap().len() as usize);
        unsafe_reader.read_to_end(&mut unsafe_bytes).expect("Failed to read unsafe file");

        for &unsafe_byte in &unsafe_bytes {
            if let Some(&value) = keys.get(&unsafe_byte) {
                processed_bytes.push(value);
            }
        }

        let encrypt_path = format!("./filecrypt/2_output_encrypt/{}.dnk", path.unwrap().file_name().to_string_lossy());
        
        fs::write(&encrypt_path, &processed_bytes).unwrap();
    }
    clearscreen::clear().unwrap();
}


pub fn decrypt_with_seed(seed: u64) {
    let keys = justify_decryption_keys(seed);
    let paths = fs::read_dir("./filecrypt/2_output_encrypt").unwrap();

    for path in paths {
        println!("Decrypting {}", path.as_ref().unwrap().path().display().to_string().bold().red());
        let mut decrypted_bytes: Vec<u8> = Vec::with_capacity(path.as_ref().unwrap().metadata().unwrap().len() as usize);
        let mut crypt_reader = File::open(path.as_ref().unwrap().path()).expect("Decrypt library failed to list");
        let mut crypt_bytes: Vec<u8> = Vec::with_capacity(path.as_ref().unwrap().metadata().unwrap().len() as usize);
        crypt_reader.read_to_end(&mut crypt_bytes).expect("Failed to read crypt");

        for crypt_byte in &crypt_bytes {
            if let Some(&value) = keys.get(&crypt_byte) {
                decrypted_bytes.push(value);
            } 

        }

        let decrypt_path = format!("./filecrypt/3_decrypted/{}", remove_extension(path.unwrap().file_name().to_string_lossy().to_string()));


        fs::write(&decrypt_path, &decrypted_bytes).unwrap();
    }
    clearscreen::clear().unwrap();
}

fn remove_extension(path: String) -> String {
    let extension = ".dnk";
    if path.ends_with(extension) {
        path[..path.len() - extension.len()].to_string()
    } else {
        path.to_string()
    }
}

pub fn justify_encryption_keys(seed: u64) -> HashMap<u8, u8> {
    let mut rng = ChaCha20Rng::seed_from_u64(seed);
    let mut keys: HashMap<u8,u8> = HashMap::new();
    let mut used: Vec<u8> = vec![];

    for index in 0..=255{ 
        let mut random = rng.gen_range(std::u8::MIN..=std::u8::MAX);
        if !used.contains(&random) {
             keys.insert(index, random);
             used.push(random);     
        } 

        else {
            loop {
                random = rng.gen_range(std::u8::MIN..=std::u8::MAX);
                if !used.contains(&random) {
                    keys.insert(index, random);
                    used.push(random);
                    break;
                }
            }        
        }
    }

    keys
}

pub fn justify_decryption_keys(seed: u64) -> HashMap<u8, u8> {
    let mut rng = ChaCha20Rng::seed_from_u64(seed);
    let mut keys: HashMap<u8,u8> = HashMap::new();
    let mut used: Vec<u8> = vec![];

    for index in 0..=255{ 
        let mut random = rng.gen_range(std::u8::MIN..=std::u8::MAX);
        if !used.contains(&random) {
             keys.insert( random, index);
             used.push(random);     
        } 

        else {
            loop {
                random = rng.gen_range(std::u8::MIN..=std::u8::MAX);
                if !used.contains(&random) {
                    keys.insert( random, index);
                    used.push(random);
                    break;
                }
            }        
        }
    }

    keys
}






/*
LEGACY EXAMPLE OF BINARYTEST

use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Read;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;


fn main() {
    let byte_debugging: bool = false;

    let unsafe_filename: &str = "./src/unsafe.png";
    let safe_filename: &str = "./src/safe.png";
    let decrypted_filename: &str = "./src/decrypted.png";    

    let mut unsafe_reader = File::open(unsafe_filename).expect("Failed to open unsafe");
    let mut unsafe_bytes = Vec::new();
    unsafe_reader.read_to_end(&mut unsafe_bytes).expect("Failed to read unsafe");


    let mut dictionary: HashMap<u8, u8> = HashMap::new();

    let mut processed_bytes: Vec<u8> = vec![];
    let mut tolerance_except: Vec<u8> = vec![];

    let mut number_generator = generate_random_byte();
    
    // Loop every byte in the unsafe read file
    for byte in &unsafe_bytes {
        // If the dictionary cant quote the byte
        if !dictionary.contains_key(&byte) {
            // Entry point of the generated hex
            let mut hex: u8 = number_generator.next().unwrap();
            // If the used hexes are not contained in the current hex, insert one
            if !tolerance_except.contains(&hex) {
                dictionary.insert(*byte, hex);
                tolerance_except.push(hex)
            } 
            
            // Else loop a tolerance of 255
            else {
                let tolerance: u8 = 255;
                
                for current_tolerance in 0..tolerance {
                    // Generate new hex and replace it
                    hex = number_generator.next().unwrap();
                    // If the generated hex is not contained in the used hexes
                    if !tolerance_except.contains(&hex) {
                        // Insert the byte and new hex
                        dictionary.insert(*byte, hex);
                        tolerance_except.push(hex);
                        break;
                        
                        // If tolerance reaches maximum; panic
                    } else if current_tolerance == tolerance -1 {
                        panic!("Maximum tolerance of {tolerance} reached")
                    }
                }
            }
        }
        processed_bytes.push(*dictionary.get(byte).unwrap());
        if byte_debugging {println!("{} {}", byte, dictionary.get(byte).unwrap());}
    }

    fs::write(safe_filename, &processed_bytes).unwrap();

   /*  ------------------------------------------------------------------------------------- */
   
    let mut safe_reader = File::open(safe_filename).expect("Failed to open safe");
    let mut safe_bytes = Vec::new();
    safe_reader.read_to_end(&mut safe_bytes).expect("Failed to read safe");


    let switched_dictionary: HashMap<u8, u8> = dictionary
    .into_iter()
    .map(|(key, value)| (value, key))
    .collect();

    let decrypted_bytes: Vec<_> = safe_bytes
    .iter()
    .map(|byte| *switched_dictionary.get(byte).unwrap())
    .collect();

    fs::write(decrypted_filename, decrypted_bytes).unwrap();
   
}

fn generate_random_byte() -> impl Iterator<Item = u8> {
    let mut rng = ChaCha20Rng::seed_from_u64(12345);
    let mut used_numbers = vec![false; 256];

    std::iter::from_fn(move || {
        let mut byte: u8;
        loop {
            byte = rng.gen();
            if !used_numbers[byte as usize] {
                used_numbers[byte as usize] = true;
                break;
            }
        }
        Some(byte)
    })
}
 */