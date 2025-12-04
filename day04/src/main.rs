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

fn read_input(file: &str) -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>> {
    let input_file = File::open(file)?;
    let buffered = BufReader::new(input_file);

    let mut data: Vec<Vec<u8>> = Vec::new();

    // Read each line from the file and parse it accordingly
    for line in buffered.lines() {
        data.push(line?.into_bytes());
    }

    Ok(data)
}

fn part1(data: &[Vec<u8>]) -> u64 {
    let width = data[0].len();
    let height = data.len();

    let mut sum: u64 = 0;

    for y in 0..height {
        let sy = y.saturating_sub(1);
        let ey = (y + 1).min(height - 1);

        for x in 0..width {
            let sx = x.saturating_sub(1);
            let ex = (x + 1).min(width - 1);

            let c = data[y][x];

            if c == b'@' {
                let mut adjacent = 0;

                'outer: for ny in sy..=ey {
                    for nx in sx..=ex {
                        if nx == x && ny == y {
                            continue;
                        }

                        if data[ny][nx] == b'@' {
                            adjacent += 1;
                            if adjacent >= 4 {
                                break 'outer;
                            }
                        }
                    }
                }
                if adjacent < 4 {
                    sum += 1;
                }
            }
        }
    }

    sum
}

fn part2(data: &[Vec<u8>]) -> u64 {
    let height = data.len();
    let width = data[0].len();

    let mut sum: u64 = 0;

    // We will forklift away rolls in part 2, so we need a mutable copy of the data
    // to be able to roll the rolls away...
    let mut grid = data.to_vec();

    // We need to reinvestigate rolls after others has been removed, but we don't want
    // to look through the entire grid all of the time. Let's use like a queue of
    // coordinates to process.
    // Initially, that queue would be all of the '@' rolls...
    let mut roll_queue: Vec<(usize, usize)> = Vec::new();
    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == b'@' {
                roll_queue.push((x, y));
            }
        }
    }

    // Let the forklifts do their thing, checking the locations in the queue one by one...
    let mut queue_index = 0;
    while queue_index < roll_queue.len() {
        let (x, y) = roll_queue[queue_index];
        queue_index += 1;

        // Could have been forklifted away already in a previous run...
        if grid[y][x] != b'@' {
            continue;
        }

        // Count neighbors to see if this one can be accessed by a forklift
        let mut adjacent = 0;
        let sy = y.saturating_sub(1);
        let ey = (y + 1).min(height - 1);
        let sx = x.saturating_sub(1);
        let ex = (x + 1).min(width - 1);

        'outer: for ny in sy..=ey {
            for nx in sx..=ex {
                if nx == x && ny == y {
                    continue;
                }
                if grid[ny][nx] == b'@' {
                    adjacent += 1;
                    if adjacent >= 4 {
                        break 'outer;   // Leave early
                    }
                }
            }
        }

        // If fewer than 4 neighbors, the elves can do their forklift thing
        if adjacent < 4 {
            grid[y][x] = b'.';
            sum += 1;

            // Neighboring rolls of this one might now be forkliftable. Add to queue!
            for ny in sy..=ey {
                for nx in sx..=ex {
                    if nx == x && ny == y {
                        continue;
                    }
                    if grid[ny][nx] == b'@' {
                        roll_queue.push((nx, ny));
                    }
                }
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests;
