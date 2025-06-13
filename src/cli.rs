use clap::{Parser, ArgAction};

/// Simple password generator in Rust
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// Length of the generated password
    #[arg(short, long, default_value_t = 16)]
    pub length: usize,

    /// Include lowercase letters
    #[arg(long, default_value_t = true, action = ArgAction::Set)]
    pub lowercase: bool,

    /// Include uppercase letters
    #[arg(long, default_value_t = true, action = ArgAction::Set)]
    pub uppercase: bool,

    /// Include digits
    #[arg(long, default_value_t = true, action = ArgAction::Set)]
    pub digits: bool,

    /// Include symbols
    #[arg(long, default_value_t = false, action = ArgAction::Set)]
    pub symbols: bool,

    /// Number of passwords to generate
    #[arg(short, long, default_value_t = 1)]
    pub count: usize,
}

pub fn parse_args() -> CliArgs {
    CliArgs::parse()
}