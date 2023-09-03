use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Read;


/* API  */

/*
    
    Single-File Encryption ->

        let enc_path_get = "C:/Users/Administrator/Documents/VisualStudioCode/todo_cli/src/testing_get/image.png"; // sane file input
        let enc_path_set = "C:/Users/Administrator/Documents/VisualStudioCode/todo_cli/src/testing_set/image.png"; // un-sane file output
        denk_filecrypt::singular_filecrypt(String::from("abcd"), &enc_path_get, &enc_path_set);

    Single-File Decryption ->
    
        let dec_path_get = "C:/Users/Administrator/Documents/VisualStudioCode/todo_cli/src/testing_set/image.png.dnk"; // un-sane file input + .dnk
        let dec_path_set = "C:/Users/Administrator/Documents/VisualStudioCode/todo_cli/src/image.png"; // sane output
        denk_filecrypt::singular_filedecrypt(String::from("abcd"), &dec_path_get, &dec_path_set);

        
    Multi-File Encryption ->

            create_folders();
            encrypt_directory_seed(String::from("PaSwsord134!")); // Takes a password (String) to encrypt
            delete_folder_one_three()
    
    Multi-File-Decryption -> 

            decrypt_directory_seed(String::from("WrongPasswordOhNo")); // Takes a password (String) to decrypt

        /* The wrong password will store just unopenable files */
        /* create_folders refreshes every content of those folders thus deletes them */
*/




fn string_to_u64(input: &str) -> u64 {
    let mut hash: u64 = 0;
    for c in input.chars() {
        hash = (hash.wrapping_mul(31)).wrapping_add(c as u64);
    }
    hash
}

fn remove_extension(path: String) -> String {
    let extension = ".dnk";
    if path.ends_with(extension) {
        path[..path.len() - extension.len()].to_string()
    } else {
        path.to_string()
    }
}

fn encrypt_directory_seed(seed: u64) {
    let keys = justify_keys(seed, true);
    let paths = fs::read_dir("./filecrypt/1_input_encrypt").unwrap();

    for path in paths {
        let mut processed_bytes: Vec<u8> =
            Vec::with_capacity(path.as_ref().unwrap().metadata().unwrap().len() as usize);
        let mut unsafe_reader =
            File::open(path.as_ref().unwrap().path()).expect("File failed to list");
        let mut unsafe_bytes =
            Vec::with_capacity(path.as_ref().unwrap().metadata().unwrap().len() as usize);
        unsafe_reader
            .read_to_end(&mut unsafe_bytes)
            .expect("Failed to read unsafe file");

        for &unsafe_byte in &unsafe_bytes {
            if let Some(&value) = keys.get(&unsafe_byte) {
                processed_bytes.push(value);
            }
        }

        let encrypt_path = format!(
            "./filecrypt/2_output_encrypt/{}.dnk",
            path.unwrap().file_name().to_string_lossy()
        );

        fs::write(&encrypt_path, &processed_bytes).unwrap();
    }
}

fn justify_keys(seed: u64, type_encryption: bool) -> HashMap<u8, u8> {
    fn skim_seed(input: u64) -> u64 {
        let mut processed = input % 10 + 3;
        if processed > 12 {
            processed -= 10;
        }

        processed *= 2;
        processed += 5;
        processed -= 7;
        processed /= 3;
        processed %= 8;
        processed += 2;
        processed *= 4;
        processed -= 10;
        processed += 6;
        processed /= 2;

        if processed > 12 || processed < 1 {
            processed %= 13;
        }

        processed
    }

    let mut rng = ChaCha20Rng::seed_from_u64(seed);
    let mut keys: HashMap<u8, u8> = HashMap::new();
    let mut used: Vec<u8> = vec![];

    let mut depth_of_field: [usize; 256] = [0; 256];
    let skimmed_number = skim_seed(seed) as usize;

    if skimmed_number > 12 {
        panic!("Skimmed number exceeds maximum of 12")
    }
    for value in depth_of_field.iter_mut() {
        *value = rng.gen_range(0..=skimmed_number);
    }

    for index in 0..=255 {
        let mut random = rng.gen_range(std::u8::MIN..=std::u8::MAX);
        for _ in 0..=depth_of_field[index as usize] {
            random = rng.gen_range(std::u8::MIN..=std::u8::MAX);
        }

        if !used.contains(&random) {
            if type_encryption {
                keys.insert(index, random);
            } else {
                keys.insert(random, index);
            }
            used.push(random);
        } else {
            loop {
                random = rng.gen_range(std::u8::MIN..=std::u8::MAX);
                if !used.contains(&random) {
                    if type_encryption {
                        keys.insert(index, random);
                    } else {
                        keys.insert(random, index);
                    }

                    used.push(random);
                    break;
                }
            }
        }
    }

    keys
}

fn decrypt_directory_seed(seed: u64) {
    let keys = justify_keys(seed, false);
    let paths = fs::read_dir("./filecrypt/2_output_encrypt").unwrap();

    for path in paths {
        let mut decrypted_bytes: Vec<u8> =
            Vec::with_capacity(path.as_ref().unwrap().metadata().unwrap().len() as usize);
        let mut crypt_reader =
            File::open(path.as_ref().unwrap().path()).expect("Decrypt library failed to list");
        let mut crypt_bytes: Vec<u8> =
            Vec::with_capacity(path.as_ref().unwrap().metadata().unwrap().len() as usize);
        crypt_reader
            .read_to_end(&mut crypt_bytes)
            .expect("Failed to read crypt");

        for crypt_byte in &crypt_bytes {
            if let Some(&value) = keys.get(&crypt_byte) {
                decrypted_bytes.push(value);
            }
        }

        let decrypt_path = format!(
            "./filecrypt/3_decrypted/{}",
            remove_extension(path.unwrap().file_name().to_string_lossy().to_string())
        );
        fs::write(&decrypt_path, &decrypted_bytes).unwrap();
    }
}

pub fn directory_filecrypt(seed: String) {
    encrypt_directory_seed(string_to_u64(&seed));
}

pub fn directory_filedecrypt(seed: String) {
    decrypt_directory_seed(string_to_u64(&seed));
}

pub fn singular_filecrypt(seed_string: String, path_get: &str, path_set: &str) {
    let seed = string_to_u64(&seed_string);
    let keys = justify_keys(seed, true);

    let mut processed_bytes: Vec<u8> =
        Vec::with_capacity(File::open(path_get).unwrap().metadata().unwrap().len() as usize);
    let mut unsafe_reader = File::open(path_get).expect("File failed to list");
    let mut unsafe_bytes =
        Vec::with_capacity(File::open(path_get).unwrap().metadata().unwrap().len() as usize);
    unsafe_reader
        .read_to_end(&mut unsafe_bytes)
        .expect("Failed to read unsafe file");

    for &unsafe_byte in &unsafe_bytes {
        if let Some(&value) = keys.get(&unsafe_byte) {
            processed_bytes.push(value);
        }
    }

    let encrypt_path = format!("{path_set}.dnk");

    fs::write(&encrypt_path, &processed_bytes).unwrap();
}

pub fn singular_filedecrypt(seed_string: String, path_get: &str, path_set: &str) {
    let seed = string_to_u64(&seed_string);
    let keys = justify_keys(seed, false);

    let mut decrypted_bytes: Vec<u8> =
        Vec::with_capacity(File::open(path_get).unwrap().metadata().unwrap().len() as usize);
    let mut crypt_reader = File::open(path_get).expect("Decrypt library failed to list");
    let mut crypt_bytes: Vec<u8> =
        Vec::with_capacity(File::open(path_get).unwrap().metadata().unwrap().len() as usize);
    crypt_reader
        .read_to_end(&mut crypt_bytes)
        .expect("Failed to read crypt");

    for crypt_byte in &crypt_bytes {
        if let Some(&value) = keys.get(&crypt_byte) {
            decrypted_bytes.push(value);
        }
    }

    let decrypt_path = remove_extension(format!("{path_set}.dnk"));
    fs::write(&decrypt_path, &decrypted_bytes).unwrap();
}

pub fn create_folders() {
    let root_folder = "filecrypt";
    let subfolder_names = ["1_input_encrypt", "2_output_encrypt", "3_decrypted"];


    if let Err(e) = fs::create_dir(root_folder) {
        if e.kind() != std::io::ErrorKind::AlreadyExists {
            eprintln!("Failed to create root folder '{}': {}", root_folder, e);
        }
    }

    for subfolder_name in &subfolder_names {
        let folder_path = format!("{}/{}", root_folder, subfolder_name);
        if let Err(e) = fs::create_dir(&folder_path) {
            if e.kind() != std::io::ErrorKind::AlreadyExists {
                eprintln!("Failed to create subfolder '{}': {}", folder_path, e);
            }
        }
    }
}

pub fn delete_folder_one_three()  {
    let input_encrypt_folder = "filecrypt/1_input_encrypt";
    let decrypted_folder = "filecrypt/3_decrypted";

    fn delete_contents(folder_path: &str) {
        for entry in fs::read_dir(folder_path).unwrap() {
            let entry = entry.unwrap();
            let entry_path = entry.path();
            if entry_path.is_dir() {
                delete_contents(&entry_path.to_string_lossy());
            } else {
                fs::remove_file(&entry_path).unwrap();
            }
        }
        
    }

    delete_contents(input_encrypt_folder);
    delete_contents(decrypted_folder);
}
