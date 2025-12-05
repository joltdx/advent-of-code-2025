use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::ops::Range;

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
            let result1 = part1(&contents.0, &contents.1);
            println!("Part 1: {}\n        {:?}", result1, start.elapsed());

            // Part 2
            let start = Instant::now();
            let result2 = part2(&contents.0, &contents.1);
            println!("\nPart 2: {}\n        {:?}", result2, start.elapsed());
        }

        // If there was an error reading the file, print an error message
        Err(e) => {
            eprintln!("Error reading input {}: {}", filename, e);
        }
    }
}

fn read_input(file: &str) -> Result<(Vec<Range<u64>>,Vec<u64>), Box<dyn std::error::Error>> {
    let input_file = File::open(file)?;
    let buffered = BufReader::new(input_file);

    let mut ranges: Vec<Range<u64>> = Vec::new();
    let mut ids: Vec<u64> = Vec::new();

    // Read each line from the file and parse it accordingly
    // Get a mutable iterator over the lines so we can read in two steps...
    let mut lines = buffered.lines();

    // Read ranges until a blank line:
    while let Some(line) = lines.next() {
        let line = line?;
        if line.is_empty() {
            break;
        }

        let (start_str, end_str) = line.split_once('-').ok_or("Invalid range format")?;
        let start: u64 = start_str.parse()?;
        let end: u64 = end_str.parse()?;
        ranges.push(start..end + 1);
    }

    // Then read IDs:
    while let Some(line) = lines.next() {
        let line = line?;
        let id: u64 = line.parse()?;
        ids.push(id);
    }

    // Ranges might be overlapping, bordering, or contained...
    // let's sort and merge them:
    ranges.sort_by(|a, b| a.start.cmp(&b.start));
    
    let mut merged_ranges: Vec<Range<u64>> = Vec::new();
    for range in ranges {
        if let Some(prev) = merged_ranges.last_mut() {
            // Overlapping or bordering?
            if range.start <= prev.end {
                if range.end > prev.end {
                    // Not contained - we extend the previous
                    prev.end = range.end;
                }
            } else {
                merged_ranges.push(range.clone());
            }
        } else {
            merged_ranges.push(range.clone());
        }
    }

    // Sort IDs as well
    ids.sort();

    Ok((merged_ranges, ids))
}

fn part1(ranges: &[Range<u64>], ids: &[u64]) -> u64 {
    let mut count_fresh: u64 = 0;

    // Ah, iterators...
    // Since we have sorted everything and removed any overlap in the ranges, we can basically 
    // now just iterate over all the ids in a for loop, while bumping up the range iterator
    // manually, as needed, and count contained ids until we either run out of ranges or ids...

    // ranges iterator handled manually...
    let mut ranges = ranges.iter();
    let mut range = match ranges.next() {
        Some(r) => r,
        None => return 0,  // No ranges at all - no ids are fresh
    };

    for id in ids {
        // Let's make sure we compare against a reasonable range. The range end being before
        // the id makes no sense, so then we move to the next range until we're cool...
        while range.end <= *id {
            match ranges.next() {
                Some(next_range) => range = next_range,
                None => return count_fresh,  // No more ranges - our work here is done
            }
        }

        // Right, so are we actually in the range?
        if range.contains(id) {
            count_fresh += 1;
        }
    }

    count_fresh
}

fn part2(ranges: &[Range<u64>], _ids: &[u64]) -> u64 {
    // wow, part 2 is just the total size of all merged ranges...
    ranges.iter().map(|r| r.end - r.start).sum()
}

#[cfg(test)]
mod tests;