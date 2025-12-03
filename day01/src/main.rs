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

fn read_input(file: &str) -> Result<Vec<(char, i32)>, Box<dyn std::error::Error>> {
    let input_file = File::open(file)?;
    let buffered = BufReader::new(input_file);

    let mut data: Vec<(char, i32)> = Vec::new();
    // Read each line from the file and parse it accordingly
    for line in buffered.lines() {
        let line = line?; // I need to bind this here so it lives longer...?
        let mut chars = line.chars();
        let direction = chars.next().ok_or("Crazy input data")?;
        let distance: i32 = chars.as_str().trim().parse()?;
        data.push((direction, distance));
    }

    Ok(data)
}

fn part1(data: &[(char, i32)]) -> i32 {
    let mut pos = 50;
    let mut count_zero = 0;

    for (direction, distance) in data {
        // Nothing very clever here... Just do the rotation and count the zero crossings
        match direction {
            'L' => {
                // Manual wrap for Left rotation
                pos -= distance;
                while pos < 0 {
                    pos += 100;
                }
            }
            'R' => {
                // For Right rotation, lets use rem_euclid, which pretty much does the
                // corresponding thing as the while loop for 'L' above, leaving us with the 
                // least non-negative remainder
                pos = (pos + distance).rem_euclid(100);
            }
            _ => {}
        }
        if pos == 0 {
            count_zero += 1;
        }
    };

    count_zero
}

fn part2(data: &[(char, i32)]) -> i32 {
    let mut pos = 50;
    let mut count_zero = 0;

    for (direction, distance) in data {
        // A bit more clever than part1... Already now count full rotations and update distance accordingly
        count_zero += distance / 100;
        let distance = distance % 100;

        match direction {
            'L' => { // Left rotation
                pos -= distance;
                if pos == 0 && distance != 0 {
                    // special case is landing on zero (but only if we actually moved more than full rotations)
                    count_zero += 1;
                } else if pos < 0 {
                    if pos + distance != 0 {
                        // special case is if we started on 0, then do not count this wrap
                        count_zero += 1;
                    }
                    pos += 100;
                }
            }
            'R' => { // Right rotation
                pos += distance;
                if pos > 99 {
                    // wrap around and a zero count
                    pos -= 100;
                    count_zero += 1;
                }
            }
            _ => {}
        }
    };
    
    count_zero
}

#[cfg(test)]
mod tests;
