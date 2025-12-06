use std::env;
use std::fs;
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
    // Just reading the lines into a vector of strings
    let content = fs::read_to_string(file)?;
    Ok(content.lines().map(String::from).collect())
}

fn part1(data: &[String]) -> u64 {
    let mut sum: u64 = 0;
    
    // We'll collect all related numbers into a vector of their own directly.
    // operator symbols are on the last line in the input and will get their own vector
    let mut numbers: Vec<Vec<u64>> = Vec::new();
    let mut symbols: Vec<char> = Vec::new();

    for line in data {
        // read all of the things on the line, and then determine if numbers or symbols...
        let all_of_the_things: Vec<&str> = line.split_whitespace().collect();
        
        if let Some(first_thing) = all_of_the_things.first() {
            if first_thing.parse::<u64>().is_ok() {
                // It's a number line
                for (i, thing) in all_of_the_things.iter().enumerate() {
                    let num = thing.parse::<u64>().expect("Failed to parse number");
                    
                    // If we haven't seen this column index before, add a new vector for it
                    if i >= numbers.len() {
                        numbers.push(Vec::new());
                    }
                    
                    numbers[i].push(num);
                }
            } else {
                // We're at the symbol line, just collect them
                symbols = all_of_the_things.iter()
                                    .filter_map(|s| s.chars().next())
                                    .collect();
            }
        }
    }

    // Compute time! iterate of the operator symbols, and just sum or product the
    // corresponding column of numbers.
    for (index, symbol) in symbols.iter().enumerate() {
        if let Some(col) = numbers.get(index) {
            match symbol {
                '+' => sum += col.iter().sum::<u64>(),
                '*' => sum += col.iter().product::<u64>(),
                _ => (),
            }
        }
    }

    sum
}

fn part2(data: &[String]) -> u64 {
    let mut sum: u64 = 0;

    // reading vertically now. with operators being at the very left of each problem,
    // we go from right to left, collecting vertical sums as we go, until we reach the
    // operator. At that point we can just calculate what we have so far...
    let width = data[0].len();
    let height = data.len();

    let mut col_sums: Vec<u64> = Vec::new();

    for x in (0..width).rev() {
        let mut col_sum: u64 = 0;
        for y in 0..height {
            let c = data[y].as_bytes()[x];
            match c {
                b' ' => (),             // space - ignore
                b'0'..=b'9' => {        // digit - build the vertical sum
                    col_sum = col_sum * 10 + (c - b'0') as u64;
                },
                b'+' | b'*' => {        // operator - compute!
                    col_sums.push(col_sum);
                    match c {
                        b'+' => sum += col_sums.iter().sum::<u64>(),
                        b'*' => sum += col_sums.iter().product::<u64>(),
                        _ => (),
                    }
                    col_sums.clear();
                    col_sum = 0;
                },
                _ => (),                // uuh, ignore other characters
            }
        }
        // if anything, collect it
        if col_sum > 0 {
            col_sums.push(col_sum);
        }
    }

    sum
}

#[cfg(test)]
mod tests;