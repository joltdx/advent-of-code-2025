use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() {
    // Get the input filename from command line arguments or default to "input.txt"
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).map(|s| s.as_str()).unwrap_or("input.txt");

    // Read the input file
    match read_input(filename) {
        // If successful, run parts 1 and 2 and measure their execution time
        Ok(contents) => {
            // Part 1
            let start = Instant::now();
            let result1 = part1(&contents);
            let runtime_ms = start.elapsed().as_nanos() as f64 / 1_000_000.0;
            println!("Part 1: {} ({:.6} ms)", result1, runtime_ms);

            // Part 2
            let start = Instant::now();
            let result2 = part2(&contents);
            let runtime_ms = start.elapsed().as_nanos() as f64 / 1_000_000.0;
            println!("Part 2: {} ({:.6} ms)", result2, runtime_ms);
        }

        // If there was an error reading the file, print an error message
        Err(e) => {
            eprintln!("Error reading input {}: {}", filename, e);
        }
    }
}

fn read_input(file: &str) -> Result<Vec<(u64, u64)>, Box<dyn std::error::Error>> {
    let input_file = File::open(file)?;
    let buffered = BufReader::new(input_file);

    let mut data: Vec<(u64, u64)> = Vec::new();

    // Read each line from the file and parse it accordingly
    for line in buffered.lines() {
        let line = line?;
        let ranges: Vec<&str> = line.trim().split(',').collect();
        for range in ranges {
            let (start_str, end_str) = range.split_once('-').ok_or("Invalid range format")?;
            let start: u64 = start_str.parse()?;
            let end: u64 = end_str.parse()?;
            data.push((start, end));
        }
    }

    Ok(data)
}

fn part1(data: &[(u64, u64)]) -> u64 {
    let mut sum_invalid: u64 = 0;

    // Finding numbers with sequence of digits repeated twice, so let's just generate
    // half numbers in each range and double them to see if they fit in the range...
    for range in data {
        // Get first half of start number
        let s_start = range.0.to_string();
        let half_start: u64 = s_start[0..s_start.len() / 2].parse().unwrap_or(0);

        // Get first half of end number
        let s_end = range.1.to_string();
        let mut half_end: u64 = s_end[0..s_end.len() / 2].parse().unwrap_or(0);

        // End half must be >= start half, so let's * 10 to make it, if not...
        if half_end < half_start {
            half_end *= 10;
        }

        // Loop through our "half number" range...
        for half_id in half_start..=half_end {
            // make a full ID from each half, and see if it's in the actual range...
            // string stuff not optimal, but let's go for this option here...
            let test_id = format!("{}{}", half_id, half_id)
                .parse::<u64>()
                .unwrap_or(0);
            if test_id >= range.0 && test_id <= range.1 {
                sum_invalid += test_id;
            }
        }
    }

    sum_invalid
}

fn part2(data: &[(u64, u64)]) -> u64 {
    let mut sum_invalid: u64 = 0;

    // Finding numbers with sequence of digits repeated _at least_ twice, so let's just generate
    // half numbers in each range, from which we generate shorter sequences to generate into full
    // numbers to see if they fit in the range...
    for range in data {
        // We could end up with duplicates here, so lets keep track in a HashSet...
        let mut invalid_ids = HashSet::<u64>::new();

        // Get first half of start number
        let s_start = range.0.to_string();
        let half_start: u64 = if s_start.len() > 1 {
            s_start[0..s_start.len() / 2].parse().unwrap_or(0)
        } else {
            range.0
        };

        // Get first half of end number
        let s_end = range.1.to_string();
        let mut half_end: u64 = if s_end.len() > 1 {
            s_end[0..s_end.len() / 2].parse().unwrap_or(0)
        } else {
            range.1
        };

        // End half must be >= start half, so let's * 10 to make it so, should that not be the case...
        if half_end < half_start {
            half_end *= 10;
        }

        // Loop through our "half numbers" range...
        for half_id in half_start..=half_end {
            let s_half = half_id.to_string();
            // Now test shorter and shorter sequences from this half...
            for len in (1..=s_half.len()).rev() {
                let sequence = &s_half[0..len];
                let seq_num = sequence.parse::<u64>().unwrap_or(0);

                // In part2, we use math instead of string stuff to generate numbers, for better performance...
                let digits = seq_num.ilog10() + 1;
                let multiplier = 10u64.pow(digits);

                // Let's generate full numbers by repeating this sequence
                // Always at least double digit...
                let mut test_id: u64 = seq_num * multiplier + seq_num;
                while test_id <= range.1 {
                    if test_id >= range.0 && test_id <= range.1 {
                        invalid_ids.insert(test_id);
                    }

                    test_id = test_id * multiplier + seq_num;
                }
            }
        }

        sum_invalid += invalid_ids.iter().sum::<u64>();
    }

    sum_invalid
}

#[cfg(test)]
mod tests;
