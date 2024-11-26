use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub path: String,

    #[arg(short, long)]
    pub min_size: Option<u64>,

    #[arg(short, long)]
    pub max_size: Option<u64>,

    #[arg(short, long)]
    pub no_cache: bool,
}

pub fn parse_args() -> Args {
    Args::parse()
}
