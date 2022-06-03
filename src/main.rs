use clap::Parser;
use histogram::{HistogramInt, Args};

fn main() {
    let args = Args::parse();
    let histo = HistogramInt::new(&args).unwrap_or_else(|err| {
        eprintln!("Application error: {}", err);
        std::process::exit(-1)
    });

    histo.statistics.print();

    histo.print_table();
}
