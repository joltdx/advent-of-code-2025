use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::collections::HashMap;

fn main() {
    // Get the input filename from command line arguments or default to "input.txt"
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).map(|s| s.as_str()).unwrap_or("input.txt");

    // Read the input file
    match read_input(filename) {
        // If successful, run parts 1 and 2 and measure their execution time
        Ok(contents) => {
            let solver = Solver::new(&contents);

            // Part 1
            let start = Instant::now();
            let result1 = solver.part1();
            println!("Part 1: {}\n        {:?}", result1, start.elapsed());

            // Part 2
            let start = Instant::now();
            let result2 = solver.part2();
            println!("\nPart 2: {}\n        {:?}", result2, start.elapsed());
        }

        // If there was an error reading the file, print an error message
        Err(e) => {
            eprintln!("Error reading input {}: {}", filename, e);
        }
    }
}

fn parse_device(s: &str) -> Device {
    let chars: Vec<char> = s.chars().collect();
    [chars[0], chars[1], chars[2]]
}

fn read_input(file: &str) -> Result<HashMap<Device, Vec<Device>>, Box<dyn std::error::Error>> {
    let input_file = File::open(file)?;
    let buffered = BufReader::new(input_file);

    let mut data: HashMap<Device, Vec<Device>> = HashMap::new();

    // Read each line from the file and parse it accordingly
    for line in buffered.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(": ").collect();
        let node = parse_device(parts[0]);
        let outputs: Vec<Device> = parts[1]
            .split_whitespace()
            .map(parse_device)
            .collect();
        data.insert(node, outputs);
    }

    Ok(data)
}

type Device = [char; 3];

struct Solver<'a> {
    graph: &'a HashMap<Device, Vec<Device>>,
}

impl<'a> Solver<'a> {
    fn new(graph: &'a HashMap<Device, Vec<Device>>) -> Self {
        Solver { graph }
    }

    fn part1(&self) -> u64 {
        self.count_the_paths(parse_device("you"), parse_device("out"), parse_device("   "), &mut HashMap::new())
    }

    fn part2(&self) -> u64 {
        // We want to get from svr to out, but via the special devices fft and dac (in any order)
        // So we count for svr->fft->dac->out and svr->dac->fft->out and add them together
        // When counting each of those, we need to avoid going through the other of the special devices

        // svr -> fft -> dac -> out
        let svr_to_fft = self.count_the_paths(parse_device("svr"), parse_device("fft"), parse_device("dac"), &mut HashMap::new());
        let fft_to_dac = self.count_the_paths(parse_device("fft"), parse_device("dac"), parse_device("   "), &mut HashMap::new());
        let dac_to_out = self.count_the_paths(parse_device("dac"), parse_device("out"), parse_device("fft"), &mut HashMap::new());
        let count_svr_fft_dac_out = svr_to_fft * fft_to_dac * dac_to_out;

        // svr -> dac -> fft -> out
        let svr_to_dac = self.count_the_paths(parse_device("svr"), parse_device("dac"), parse_device("fft"), &mut HashMap::new());
        let dac_to_fft = self.count_the_paths(parse_device("dac"), parse_device("fft"), parse_device("   "), &mut HashMap::new());
        let fft_to_out = self.count_the_paths(parse_device("fft"), parse_device("out"), parse_device("dac"), &mut HashMap::new());
        let count_svr_dac_fft_out = svr_to_dac * dac_to_fft * fft_to_out;

        count_svr_fft_dac_out + count_svr_dac_fft_out
    }

    fn count_the_paths(&self, from: Device, to: Device, avoid: Device, memo: &mut HashMap<Device, u64>) -> u64 {
        // We want to avoid this one
        if from == avoid {
            return 0;
        }

        // we've reached our goal
        if from == to {
            return 1;
        }
        
        // Do we already have a memoized count for this node?
        if let Some(&count) = memo.get(&from) {
            return count;
        }

        // We need to start counting paths from here
        let mut path_count = 0;
        if let Some(outputs) = self.graph.get(&from) {
            for &output in outputs {
                path_count += self.count_the_paths(output, to, avoid, memo);
            }
        }

        // Memoize the count for this node
        memo.insert(from, path_count);
        path_count
    }
}

#[cfg(test)]
mod tests;