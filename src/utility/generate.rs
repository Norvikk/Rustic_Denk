use rayon::prelude::*;

pub fn random_string(size: usize) -> String {
    let options_chars: [char; 89] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
        't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '!', '@',
        '#', '$', '%', '^', '&', '*', '(', ')', '_', '+', '-', '=', '[', ']', '{', '}', '|', ';',
        ':', ',', '.', '/', '<', '>', '?', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K',
        'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

     (0..size)
        .into_par_iter()
        .map(|_| {
            let rng = fastrand::Rng::new();
            options_chars[rng.usize(0..options_chars.len())]
        })
        .collect()
}




pub fn custom_random_string( lower_letters: bool, numbers: bool, symbols: bool,upper_letters: bool,
    size: usize,
) -> String {
    let rng = fastrand::Rng::new();
    let mut result = String::with_capacity(size);

    let mut options = Vec::new();
    if lower_letters {
        options.push("abcdefghijklmnopqrstuvwxyz");
    }
    if numbers {
        options.push("0123456789");
    }
    if symbols {
        options.push("!@#$%^&*()_+-=[]{}|;':,./<>?");
    }
    if upper_letters {
        options.push("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }

    if options.is_empty() {
        panic!("At least one option must be selected");
    }

    let options_chars: Vec<char> = options.iter().flat_map(|&s| s.chars()).collect();

    for _ in 0..size {
        let random_index = rng.usize(0..options_chars.len());
        result.push(options_chars[random_index]);
    }

    result
}