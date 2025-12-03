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
            println!("Part 1: {}\n        {:?}", result1, start.elapsed());

            // Part 2
            let start = Instant::now();
            let result2 = part2(&contents);
            println!("\nPart 2: {}\n        {:?}", result2, start.elapsed());
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
    data.iter().count() as u64
}

fn part2(data: &[String]) -> u64 {
    data.iter().count() as u64
}

#[cfg(test)]
mod tests;