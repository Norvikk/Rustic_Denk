
use rand::Rng;
use rand::SeedableRng;
pub fn recentralize(text: &str, shift: u8) -> String {
    let seed: u32 = 34591;
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed as u64);

    text.chars()
        .map(|c| {
            
                let base = rng.gen_range(0..10);
                let offset = (c as u8 + base - shift) % 26;
                (base + offset) as char
            
        })
        .collect()

    
}