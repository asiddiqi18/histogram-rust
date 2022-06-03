#[macro_use]
extern crate prettytable;

use clap::Parser;
use colored::*;
use prettytable::format;
use prettytable::Table;
use std::cmp;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::num::ParseIntError;

/// Simple program to print a histogram from a CSV file
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Name of CSV file to read
    filename: String,

    /// Number of bins for the histogram
    bin_count: i32,

    /// Starting range for the histogram
    #[clap(short, long)]
    starting_range: Option<i32>,

    /// Ending range for the histogram
    #[clap(short, long)]
    ending_range: Option<i32>,

    /// Maximum block count to show per row before scaling down
    #[clap(short, long, default_value_t = 30)]
    max_blocks: i32,
}

pub struct Statistics {
    pub mean: f32,
    pub mode: i32,
    pub mode_count: i32,
    pub size_of_data: i32,
    pub sum: i32,
    pub min: i32,
    pub max: i32,
    pub num_freq_map: HashMap<i32, i32>,
}

impl Statistics {
    pub fn new() -> Statistics {
        Statistics {
            mean: 0.0,
            mode: 0,
            mode_count: 0,
            size_of_data: 0,
            sum: 0,
            min: i32::MAX,
            max: i32::MIN,
            num_freq_map: HashMap::new(),
        }
    }

    pub fn print(&self) {
        println!("--------Statistics-------");
        println!("Size: {:?}", self.size_of_data);
        println!("Mean: {:?}", self.mean);
        println!("Mode: {:?} (appears {} times)", self.mode, self.mode_count);
        println!("Sum: {:?}", self.sum);
        println!("Min: {:?}", self.min);
        println!("Max: {:?}", self.max);
    }
}

pub struct HistogramInt {
    pub data: Vec<i32>,
    pub bin_width: i32,
    pub starting_range: i32,
    pub ending_range: i32,
    pub statistics: Statistics,
    max_blocks: i32,
}

impl HistogramInt {
    pub fn print_table(&self) {
        let mut table = Table::new();

        let no_range = self.bin_width == 1;

        let largest_bin_qty = match self.data.iter().max() {
            Some(min) => *min,
            None => 1,
        };

        let scale = 1 + (largest_bin_qty / (self.max_blocks + 1));

        if no_range {
            table.set_titles(row![H3c->"Histogram"]);
        } else {
            table.set_titles(row![H5c->"Histogram"]);
        }

        for (i, item) in self.data.iter().enumerate() {
            let bin_start = (self.starting_range) + (i as i32 * self.bin_width);
            let bin_end = bin_start + self.bin_width - 1;

            let repeat_amount = *item / scale;

            if no_range {
                table.add_row(row![
                    bin_start,
                    Fg->"\u{25A0}".repeat(repeat_amount as usize),
                    format!("({})", *item)
                ]);
            } else {
                table.add_row(row![
                    bin_start,
                    '-',
                    bin_end,
                    Fg->"\u{25A0}".repeat(repeat_amount as usize),
                    format!("({})", *item)
                ]);
            }
        }

        table.set_format(*format::consts::FORMAT_CLEAN);
        table.printstd();
        println!("Scale: {} = {}", "\u{25A0}".green(), scale)
    }
}

pub fn get_statistics<'a>(contents: &'a str) -> Result<Statistics, ParseIntError> {
    let mut stats = Statistics::new();

    for (line_index, line) in contents.lines().enumerate() {
        let mut left_ptr = 0;
        for (chr_index, chr) in line.chars().enumerate() {
            if chr == ',' {
                let val = &line[left_ptr..chr_index].trim_start();
                left_ptr = chr_index + 1;
                parse_word(val, &mut stats, line_index, chr_index)?;
            }
        }

        if left_ptr != line.len() {
            let val = &line[left_ptr..].trim();
            if !val.is_empty() {
                parse_word(val, &mut stats, line_index, left_ptr)?;
            }
        }
    }

    stats.mean = stats.sum as f32 / stats.size_of_data as f32;

    Ok(stats)
}

fn parse_word(
    val: &str,
    stats: &mut Statistics,
    line_index: usize,
    char_index: usize,
) -> Result<(), ParseIntError> {
    let num = match val.parse::<i32>() {
        Ok(num) => num,
        Err(err) => {
            eprintln!(
                "Couldn't parse as an integer at line {}, position {} ({})",
                line_index, char_index, err
            );
            return Err(err);
        }
    };

    stats.size_of_data += 1;
    stats.sum += num;
    stats.max = cmp::max(stats.max, num);
    stats.min = cmp::min(stats.min, num);

    let count = stats.num_freq_map.entry(num).or_insert(0);
    *count += 1;

    if *count > stats.mode_count {
        stats.mode_count = *count;
        stats.mode = num;
    };

    Ok(())
}

pub fn get_histogram(args: &Args) -> Result<HistogramInt, Box<dyn Error>> {
    let contents = fs::read_to_string(args.filename.clone())?;
    let statistics = get_statistics(&contents)?;

    let starting_range = match args.starting_range {
        Some(val) => val,
        None => statistics.min,
    };
    let ending_range = match args.ending_range {
        Some(val) => val,
        None => statistics.max,
    };

    if starting_range > statistics.max {
        return Err("Starting range is greater than the maximum value present in data.".into());
    } else if ending_range < statistics.min {
        return Err("Ending range is less than the minimum value present in data.".into());
    } else if starting_range >= ending_range {
        return Err("Starting range must be less than the ending range.".into());
    }

    let mut v = vec![0; args.bin_count.try_into().unwrap()];
    let bin_width = (ending_range - starting_range + args.bin_count) / args.bin_count;

    for (num, freq) in &statistics.num_freq_map {
        if *num < starting_range || *num > ending_range {
            continue;
        }

        let belongs_to_bin_index = ((num - starting_range) / bin_width) as usize;
        let current = v.get(belongs_to_bin_index).expect(&format!(
            "Index {}, number {} - {}",
            belongs_to_bin_index, num, "Accessed out of bounds."
        ));

        v[belongs_to_bin_index] = current + freq;
    }

    Ok(HistogramInt {
        data: v,
        bin_width,
        starting_range,
        ending_range,
        statistics,
        max_blocks: args.max_blocks,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uniform_10000_range_1_100_bin_10() {
        let args = Args {
            filename: String::from("tests/test_uniform_10000.csv"),
            bin_count: 10,
            starting_range: Some(1),
            ending_range: Some(10000),
            max_blocks: 30,
        };

        let histo = get_histogram(&args).unwrap();

        assert_eq!([1000].repeat(10), histo.data);
    }

    #[test]
    fn test_uniform_10000_range_1_100_bin_100() {
        let args = Args {
            filename: String::from("tests/test_uniform_10000.csv"),
            bin_count: 100,
            starting_range: Some(1),
            ending_range: Some(10000),
            max_blocks: 30,
        };

        let histo = get_histogram(&args).unwrap();
        assert_eq!([100].repeat(100), histo.data);
    }

    #[test]
    fn test_uniform_10000_no_range_specified_bin_10() {
        let args = Args {
            filename: String::from("tests/test_uniform_10000.csv"),
            bin_count: 10,
            starting_range: None,
            ending_range: None,
            max_blocks: 30,
        };

        let histo = get_histogram(&args).unwrap();
        assert_eq!([1000].repeat(10), histo.data);
    }

    #[test]
    fn test_uniform_10000_starting_larger_than_ending() {
        let args = Args {
            filename: String::from("tests/test_uniform_10000.csv"),
            bin_count: 10,
            starting_range: Some(10000),
            ending_range: Some(1),
            max_blocks: 30,
        };

        let histo = get_histogram(&args);
        assert!(histo.is_err())
    }
}
