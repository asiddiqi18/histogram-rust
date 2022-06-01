use std::env;
use std::process;
use histogram::Config;

pub mod random_data;

fn main() {

    random_data::generate("binomial.csv").unwrap_or_else(|err| {
        println!("Error generating file: {}", err);
        std::process::exit(-1)
    });

    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    config.print();

    let histo = histogram::get_histogram(&config).unwrap_or_else(|err| {
        println!("Application error: {}", err);
        std::process::exit(-1)
    });

    histo.statistics.print();

    histo.print();

}
