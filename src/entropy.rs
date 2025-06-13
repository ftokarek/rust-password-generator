use std::collections::HashSet;

/// Calculates and prints the entropy and strength of the password(s).
pub fn calculate_entropy(passwords: &[String], charset: &str) 
{
    let charset_size = charset.chars().collect::<HashSet<_>>().len();
    if charset_size == 0 {
        println!("Cannot calculate entropy: charset is empty.");
        return;
    }

    let log2_charset = (charset_size as f64).log2();

    for (i, password) in passwords.iter().enumerate() 
    {
        let entropy = password.len() as f64 * log2_charset;
        let strength = match entropy 
        {
            e if e < 40.0 => "Weak",
            e if e < 60.0 => "Medium",
            _ => "Strong",
        };
        println!(
            "Password {}: entropy = {:.2} bits ({})",
            i + 1,
            entropy,
            strength
        );
    }
}