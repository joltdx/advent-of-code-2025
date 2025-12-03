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

fn read_input(file: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let input_file = File::open(file)?;
    let buffered = BufReader::new(input_file);

    let mut data: Vec<String> = Vec::new();

    // Read each line from the file and parse it accordingly
    for line in buffered.lines() {
        data.push(line?);
    }

    Ok(data)
}

fn part1(data: &[String]) -> u64 {
    let mut joltage: u64 = 0;

    // Experimenting and learning as I go here :)
    // Not very efficient, actually runs slower than part2, and I think harder to read also, but it is what is is...
    // (Also not very safe for utf-8 and such, but input is considered valid and consistent...)

    for bank in data {
        // Find the index and character of the maximum digit in the bank string, excluding the last character...
        // max finds the _last_ maximum, so need to do a rev() to find the first...
        let (max_first_i, max_first_c) = bank[0..bank.len() - 1].char_indices().rev().max_by_key(|&(_, c)| c).unwrap();
        
        // convert char to string and the parse into an u64. Not very efficient at all, lol
        let max_first: u64 = max_first_c.to_string().parse::<u64>().unwrap();

        // now we just need the max of the remaining characters (no need to reverse), string and parse it to u64 
        let max_second: u64 = bank[max_first_i + 1..].chars().max().unwrap().to_string().parse::<u64>().unwrap();

        // make it a joltage number
        let max = max_first * 10 + max_second;

        joltage += max;
    }

    joltage
}

fn part2(data: &[String]) -> u64 {
    let mut joltage: u64 = 0;

    // Right, so let use bytes in part2 for higher brrrrr

    for bank in data {
        let bytes = bank.as_bytes();
        
        let mut bank_joltage: u64 = 0;
        let mut start_index = 0;
        
        // starting from the left, we need to find 12 maximum digits...
        for digit_number in (0..12).rev() {
            let mut max_digit = 0u8;
            let mut max_index = 0;
        
            // we always must keep enough digits to the right to be able to complete the number
            // and we can't use any digits to the left of the last found max...
            for (i, &b) in bytes[start_index..bytes.len() - digit_number].iter().enumerate() {
                // new max?
                if b > max_digit {
                    max_digit = b;
                    max_index = i;
                    // early exit if we found a 9 already
                    if max_digit == b'9' {
                        break;
                    }
                }
            }
            start_index += max_index + 1;

            // byte to digit and build up the banks joltage number
            bank_joltage = bank_joltage * 10 + (max_digit - b'0') as u64;
        }

        // add to total joltage
        joltage += bank_joltage;
    }

    joltage
}

#[cfg(test)]
mod tests;