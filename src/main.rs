mod cli;
mod generator;
mod entropy;
mod output;

fn main() {
    let args = cli::parse_args();
    println!("Selected password length: {}", args.length);
    println!("Include lowercase: {}", args.lowercase);
    println!("Include uppercase: {}", args.uppercase);
    println!("Include digits: {}", args.digits);
    println!("Include symbols: {}", args.symbols);
    println!("Number of passwords: {}", args.count);

    generator::generate_password();
    entropy::calculate_entropy();
    output::handle_output();
}