use rand_distr::{Binomial, Distribution};
use std::fs::File;
use std::io::Write;

pub fn generate(filename: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(filename)?;

    let number_of_trials = 20;
    let probability_of_success = 0.5;
    let bin = Binomial::new(number_of_trials, probability_of_success).unwrap();
    let mut rng = rand::thread_rng();

    let mut result_string = String::new();

    for _ in 0..100 {
        let rand_num: u64 = bin.sample(&mut rng);
        result_string.push_str(&rand_num.to_string());
        result_string.push(',');
    }

    result_string.pop();

    write!(file, "{}", result_string)?;
    Ok(())
}
