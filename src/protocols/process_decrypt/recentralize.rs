use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

pub fn recentralize(input: &str) -> String {
    let seed: u64 = 54628;
    let mut rng = StdRng::seed_from_u64(seed);

    let mut output = String::new();

    for c in input.chars() {
        let mut new_char = c as u8;
        let shift = rng.gen_range(1..10) as u8;
        new_char = new_char.wrapping_sub(shift);
        output.push(new_char as char);
    }
    

    output
}