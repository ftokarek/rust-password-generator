use rand::{rng, Rng};

pub fn generate_password(length: usize, charset: &str) -> String {
    let mut rng = rng();
    let chars: Vec<char> = charset.chars().collect();
    (0..length)
        .map(|_| 
        {
            let idx = rng.random_range(0..chars.len());
            chars[idx]
        })
        .collect()
}