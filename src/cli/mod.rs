use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to the input image
    #[arg(short, long)]
    pub input: PathBuf,

    /// Number of colors to extract
    #[arg(short, long, default_value_t = 5)]
    pub num_colors: usize,

    /// Output format (json or text)
    #[arg(short, long, default_value = "text")]
    pub format: String,

    /// Output file path (optional)
    #[arg(short, long)]
    pub output: Option<PathBuf>,
} 