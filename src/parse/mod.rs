use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    author = "Deniz Hofmeister",
    version = "1.0.0-rc1",
    about = "File System Monitor"
)]
pub struct Args {
    #[arg(short = 'p', long, num_args = 1.., trailing_var_arg = true)]
    pub paths: Vec<String>,
}

pub fn parse_args() -> Args {
    Args::parse()
}
