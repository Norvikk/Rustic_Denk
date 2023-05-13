use rand::Rng;

pub fn random_string(lower_letters: bool, numbers: bool, symbols: bool, upper_letters: bool, size: usize) -> String {
    let mut result = String::new();
    let mut rng = rand::thread_rng();

    let mut options = Vec::new();
    if lower_letters { options.push("abcdefghijklmnopqrstuvwxyz"); }
    if numbers { options.push("0123456789"); }
    if symbols { options.push("!@#$%^&*()_+-=[]{}|;':,./<>?"); }
    if upper_letters { options.push("ABCDEFGHIJKLMNOPQRSTUVWXYZ"); }

    if options.is_empty() {
        panic!("At least one option must be selected");
    }

    let options_chars: Vec<char> = options
    .iter()
    .flat_map(|&s| s.chars())
    .collect();

  

    
   
    for _ in 0..size {
        
        result.push(options_chars[rng.gen_range(0..options_chars.len())]);
    }

    result
}
