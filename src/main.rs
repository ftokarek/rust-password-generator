mod cli;
mod generator;
mod entropy;
mod output;

use std::io::{self, Write};
use std::env;

fn main() 
{
    // Get arguments, skip program name
    let mut args = env::args();
    let _program = args.next();

    // If there are any arguments, use CLI mode; otherwise, use interactive mode
    if args.len() > 0 
    {
        // CLI mode
        let cli_args = cli::parse_args();

        let mut charset = String::new();
        if cli_args.lowercase 
        {
            charset.push_str("abcdefghijklmnopqrstuvwxyz");
        }
        if cli_args.uppercase 
        {
            charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        }
        if cli_args.digits 
        {
            charset.push_str("0123456789");
        }
        if cli_args.symbols 
        {
            charset.push_str("!@#$%^&*()-_=+[]{};:,.<>?/|");
        }

        if charset.is_empty() 
        {
            eprintln!("Error: No character sets selected!");
            return;
        }

        let mut passwords = Vec::new();
        for _ in 0..cli_args.count 
        {
            let password = generator::generate_password(cli_args.length, &charset);
            passwords.push(password);
        }
        output::print_passwords(&passwords);
        entropy::calculate_entropy(&passwords, &charset);
    } 
    else 
    {
        // Interactive mode
        println!("rust-password-generator - Secure password generator CLI");
        println!("(Interactive mode)");

        print!("Enter password length (default 16): ");
        io::stdout().flush().unwrap();
        let mut length_input = String::new();
        io::stdin().read_line(&mut length_input).unwrap();
        let length = length_input.trim().parse::<usize>().unwrap_or(16);

        print!("Enter number of passwords to generate (default 1): ");
        io::stdout().flush().unwrap();
        let mut count_input = String::new();
        io::stdin().read_line(&mut count_input).unwrap();
        let count = count_input.trim().parse::<usize>().unwrap_or(1);

        println!("Include character sets? (y/n, default: y)");
        print!("Lowercase letters? ");
        io::stdout().flush().unwrap();
        let mut lowercase_input = String::new();
        io::stdin().read_line(&mut lowercase_input).unwrap();
        let lowercase = lowercase_input.trim().to_lowercase() != "n";

        print!("Uppercase letters? ");
        io::stdout().flush().unwrap();
        let mut uppercase_input = String::new();
        io::stdin().read_line(&mut uppercase_input).unwrap();
        let uppercase = uppercase_input.trim().to_lowercase() != "n";

        print!("Digits? ");
        io::stdout().flush().unwrap();
        let mut digits_input = String::new();
        io::stdin().read_line(&mut digits_input).unwrap();
        let digits = digits_input.trim().to_lowercase() != "n";

        print!("Symbols? ");
        io::stdout().flush().unwrap();
        let mut symbols_input = String::new();
        io::stdin().read_line(&mut symbols_input).unwrap();
        let symbols = symbols_input.trim().to_lowercase() == "y";
        let mut charset = String::new();
        if lowercase 
        {
            charset.push_str("abcdefghijklmnopqrstuvwxyz");
        }
        if uppercase 
        {
            charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        }
        if digits 
        {
            charset.push_str("0123456789");
        }
        if symbols 
        {
            charset.push_str("!@#$%^&*()-_=+[]{};:,.<>?/|");
        }

        if charset.is_empty() 
        {
            eprintln!("Error: No character sets selected!");
            return;
        }

        let mut passwords = Vec::new();
        for _ in 0..count 
        {
            let password = generator::generate_password(length, &charset);
            passwords.push(password);
        }

        output::print_passwords(&passwords);
        entropy::calculate_entropy(&passwords, &charset);
    }
}