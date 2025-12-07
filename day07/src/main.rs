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

            // We start at the 'S' on the first line...
            let start_pos = match contents[0].iter().position(|&c| c == b'S') {
                Some(pos) => pos,
                None => panic!("Hey we need an 'S' on the first line to start"),
            };

            // And let's skip the first line in the solver data, now that we've got the start pos
            let mut solver: Solver = Solver::new(&contents[1..]);

            // Part 1
            let start = Instant::now();
            let result1 = solver.part1(start_pos); 
            println!("Part 1: {}\n        {:?}", result1, start.elapsed());

            // Part 2
            let start = Instant::now();
            let result2 = solver.part2(start_pos); 
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
    // We'll get by with skipping all the "empty" lines containing only .......
    for line in buffered.lines().step_by(2) {
        data.push(line?.into_bytes());
    }

    Ok(data)
}

// For part 2, instead of having global data (for the recursive and memoization stuff)
// let's use a struct + impl to hold the things...
// (And I also refactored part 1 into here)
struct Solver<'a> {
    grid: &'a [Vec<u8>],
    width: usize,
    height: usize,
    memoized: Vec<Option<u64>>, // More efficient than a HashMap for this case
}

impl<'a> Solver<'a> {
    fn new(grid: &'a [Vec<u8>]) -> Self {
        let width = grid[0].len();
        let height = grid.len();
        Self {
            grid,
            width,
            height,
            memoized: vec![None; width * height],
        }
    }

    fn part1(&self, start_pos: usize) -> u64 {
        let mut split_count = 0;

        // Data is relatively narrow, we can just track beams as a vector of booleans,
        // for better performance than like a HashSet, that I tried first...
        let mut beams = vec![false; self.width];
        let mut new_beams = vec![false; self.width];
            
        beams[start_pos] = true;

        // Loop the lines and count the splits
        for line in self.grid.iter() { 
            new_beams.fill(false);

            // we _are_ checking all x positions, but with the beams boolean vector
            // this is still pretty performant...
            for i in 0..self.width {
                if beams[i] {
                    if line[i] == b'^' {
                        // Splitter, new beams are to left and right
                        if i > 0 {
                            new_beams[i - 1] = true;
                        }
                        if i + 1 < self.width {
                            new_beams[i + 1] = true;
                        }
                        split_count += 1;
                    } else {
                        // Empty space, this one continues down
                        new_beams[i] = true;
                    }
                }
            }

            // swap the old and new beam vectors for next line...
            std::mem::swap(&mut beams, &mut new_beams);
        }

        split_count
    }

    fn part2(&mut self, start_pos: usize) -> u64 {
        self.count_quantum_paths(start_pos, 0)
    }

    fn count_quantum_paths(&mut self, x: usize, y: usize) -> u64 {
        // We've reached the end - we count 1...
        if y >= self.height {
            return 1;
        }

        // Memoization go brrrrr
        let memo_index = y * self.width + x;
        if let Some(val) = self.memoized[memo_index] {
            // We already had it
            return val;
        }

        let timeline_count = match self.grid[y][x] {
            b'^' => {           // Splitter, we count both left and right here...
                let left = if x > 0 {
                    self.count_quantum_paths(x - 1, y + 1)
                } else {
                    0
                };

                let right = if x + 1 < self.width {
                    self.count_quantum_paths(x + 1, y + 1)
                } else {
                    0
                };

                left + right
            }

            _ => self.count_quantum_paths(x, y + 1) // Empty space, just continue down          
            
        };

        // We know it now, keep also for later
        self.memoized[memo_index] = Some(timeline_count);

        timeline_count
    }
}

#[cfg(test)]
mod tests;
