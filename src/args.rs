use clap::Parser;

/// Simple program to print a histogram from a CSV file
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Name of CSV file to read
    pub filename: String,

    /// Number of bins for the histogram
    pub bin_count: i32,

    /// Starting range for the histogram
    #[clap(short, long)]
    pub starting_range: Option<i32>,

    /// Ending range for the histogram
    #[clap(short, long)]
    pub ending_range: Option<i32>,

    /// Maximum block count to show per row before scaling down
    #[clap(short, long, default_value_t = 30)]
    pub max_blocks: i32,
}