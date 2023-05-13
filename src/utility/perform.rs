pub fn split_string_into_chunks(s: &String, n: usize) -> Vec<String> {
    s.chars()
        .collect::<Vec<char>>()
        .chunks(n)
        .map(|chunk| chunk.iter().collect())
        .collect::<Vec<String>>()
}

