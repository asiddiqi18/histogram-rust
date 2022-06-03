# Histogram-Rust
This is a simple command-line tool implemented with Rust to generate 
histograms and other statistics from a [CSV file](https://en.wikipedia.org/wiki/Comma-separated_values).

Inputs for the CSV file must only consist of integers, whitespaces, and commas as delimters.

This project was primarily a way for me to familiarize myself with Rust.

# Screenshot
The below screenshot displays the command in use. A file in the path `tests/geo_test.csv` is being read 
with `21` bins 
and a starting range of `0` 
and an ending range of `20`.

<img src="https://user-images.githubusercontent.com/60483997/171786052-0c129d2c-0cdb-4478-94da-9ebc1468ae49.png" width="400" height="450">


# Usage
```
histogram 0.1.0
MarCarrot
Simple program to print a histogram from a CSV file

USAGE:
    histogram [OPTIONS] <FILENAME> <BIN_COUNT>

ARGS:
    <FILENAME>     Name of CSV file to read
    <BIN_COUNT>    Number of bins for the histogram

OPTIONS:
    -e, --ending-range <ENDING_RANGE>
            Ending range for the histogram

    -h, --help
            Print help information

    -m, --max-blocks <MAX_BLOCKS>
            Maximum block count to show per row before scaling down [default: 30]

    -s, --starting-range <STARTING_RANGE>
            Starting range for the histogram

    -V, --version
            Print version information

```
