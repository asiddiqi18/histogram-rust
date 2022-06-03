use clap::Parser;
use histogram::Args;

fn main() {
    let args = Args::parse();
    let histo = histogram::get_histogram(&args).unwrap_or_else(|err| {
        eprintln!("Application error: {}", err);
        std::process::exit(-1)
    });

    histo.statistics.print();

    histo.print_table();
}
