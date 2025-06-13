use clap::{Parser, ArgAction};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs 
{
    #[arg(short, long, default_value_t = 16)]
    pub length: usize,

    #[arg(long, default_value_t = true, action = ArgAction::Set)]
    pub lowercase: bool,

    #[arg(long, default_value_t = true, action = ArgAction::Set)]
    pub uppercase: bool,

    #[arg(long, default_value_t = true, action = ArgAction::Set)]
    pub digits: bool,

    #[arg(long, default_value_t = false, action = ArgAction::SetTrue)]
    pub symbols: bool,

    #[arg(short, long, default_value_t = 1)]
    pub count: usize,
}

pub fn parse_args() -> CliArgs {
    CliArgs::parse()
}