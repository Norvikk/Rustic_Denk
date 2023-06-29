use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::ops::Index;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use std::path::Path;


pub fn string_to_u32(input: &str) -> u32 {
    let mut hash: u32 = 0;

    for c in input.chars() {
        hash = (hash.wrapping_mul(31)).wrapping_add(c as u32);
    }

    hash
}

pub fn encrypt_with_seed(seed: u64) {
    let keys = justify_keys(seed);
    let paths = fs::read_dir("./input_encrypt").unwrap();

    for path in paths {
        let mut processed_bytes: Vec<u8> = vec![];
        let index_path = path.unwrap().path();
        let mut unsafe_reader = File::open(&index_path).expect("Library failed to list");
        let mut unsafe_bytes = Vec::new();
        unsafe_reader.read_to_end(&mut unsafe_bytes).expect("Failed to read unsafe");

        for byte in &unsafe_bytes {
            processed_bytes.push(keys[*byte as usize]);
        }

        // for index in 0..100 {
        //     println!("{} ><>< {}", &unsafe_bytes[index], &processed_bytes[index])
        // }

        fs::write( "./output_encrypt/test.dnk", processed_bytes).unwrap();
        format!("./output_encrypt/{}.dnk", Path::new(&index_path)
        .file_stem().and_then(|stem| stem.to_str()).unwrap()),format!("{:?}"
    }
}


pub fn justify_keys(seed: u64) -> [u8; 256] {
    let mut rng = ChaCha20Rng::seed_from_u64(seed);
    let mut keys: [u8; 256] = [0; 256];

    for index in 0..256{ keys[index] = rng.gen_range(std::u8::MIN..std::u8::MAX); }

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