use colored::*;
use std::cmp;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::num::ParseIntError;

pub struct Config {
    pub filename: String,
    pub bins: i32,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, Box<dyn Error>> {
        if args.len() < 3 {
            return Err("not enough arguments".into());
        }

        let filename = args[1].clone();
        let bins = args[2].clone().parse::<i32>()?;

        Ok(Config { filename, bins })
    }

    pub fn print(&self) {
        println!("-----------Info----------");
        println!("Creating histogram for file {}", self.filename);
        println!(" > Bin count as {}", self.bins);
    }
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
    pub bin_width: f32,
    pub starting_range: i32,
    pub statistics: Statistics,
}

impl HistogramInt {
    pub fn print(&self) {
        println!("--------Histogram--------");
        for (i, item) in self.data.iter().enumerate() {
            let bin_start = (self.starting_range as usize) + (i * self.bin_width as usize);
            let bin_end = bin_start + self.bin_width as usize - 1;
            let range = format!("{}-{}", bin_start as usize, bin_end);

            println!(
                "{0: <10} {1} ({2})",
                range,
                "\u{25A0}".repeat(*item as usize).green(),
                *item
            );
        }
    }
}

pub fn get_statistics<'a>(contents: &'a str) -> Result<Statistics, ParseIntError> {
    let mut stats = Statistics::new();

    for line in contents.lines() {
        let v: Vec<&str> = line.split(|c| c == ',' || c == ' ').collect();

        let mut i = 0;
        while i < v.len() {
            if v[i] != "" {
                let num = v[i].parse::<i32>()?;

                stats.size_of_data += 1;
                stats.sum += num;
                stats.max = cmp::max(stats.max, num);
                stats.min = cmp::min(stats.min, num);

                let count = stats.num_freq_map.entry(num).or_insert(0);
                *count += 1;

                if *count > stats.mode_count {
                    stats.mode_count = *count;
                    stats.mode = num;
                }
            }
            i += 1;
        }
    }

    stats.mean = stats.sum as f32 / stats.size_of_data as f32;

    Ok(stats)
}

pub fn get_histogram(config: &Config) -> Result<HistogramInt, Box<dyn Error>> {

    let contents = fs::read_to_string(config.filename.clone())?;

    let statistics = get_statistics(&contents)?;

    let mut v = vec![0; config.bins.try_into().unwrap()];

    let bin_width = ((statistics.max - statistics.min) as f32 / config.bins as f32).ceil();

    for (num, freq) in &statistics.num_freq_map {
        let belongs_to_bin_index = ((num - statistics.min) as f32 / bin_width).floor() as usize;

        let current = v
            .get(belongs_to_bin_index)
            .expect("Accessed out of bounds.");

        v[belongs_to_bin_index] = current + freq;
    }

    Ok(HistogramInt {
        data: v,
        bin_width,
        starting_range: statistics.min,
        statistics,
    })
}
