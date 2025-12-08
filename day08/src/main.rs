use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

struct JunctionBox {
    id: usize,
    x: u64,
    y: u64,
    z: u64,
}

impl JunctionBox {
    fn new(id: usize, x: u64, y: u64, z: u64) -> Self {
        JunctionBox { id, x, y, z }
    }
}

fn main() {
    // Get the input filename from command line arguments or default to "input.txt"
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).map(|s| s.as_str()).unwrap_or("input.txt");

    // Read the input file
    match read_input(filename) {
        // If successful, run parts 1 and 2 and measure their execution time
        Ok(mut contents) => {
            let mut solver: Solver = Solver::new(&mut contents);

            let initial_connection_count = match filename.contains("test") {
                true => 10,
                false => 1000,
            };

            // Part 1
            let start = Instant::now();
            let result1 = solver.part1(initial_connection_count);
            println!("Part 1: {}\n        {:?}", result1, start.elapsed());

            // Part 2
            let start = Instant::now();
            let result2 = solver.part2(initial_connection_count);
            println!("\nPart 2: {}\n        {:?}", result2, start.elapsed());
        }

        // If there was an error reading the file, print an error message
        Err(e) => {
            eprintln!("Error reading input {}: {}", filename, e);
        }
    }
}

fn read_input(file: &str) -> Result<Vec<JunctionBox>, Box<dyn std::error::Error>> {
    let input_file = File::open(file)?;
    let buffered = BufReader::new(input_file);

    let mut data: Vec<JunctionBox> = Vec::new();
    // Read each line from the file and parse it accordingly
    for (id, line) in buffered.lines().enumerate() {
        let line = line?;
        let coordinates: Vec<u64> = line
            .split(',')
            .map(|s| s.trim().parse().unwrap_or(0))
            .collect();
        if coordinates.len() == 3 {
            data.push(JunctionBox::new(
                id,
                coordinates[0],
                coordinates[1],
                coordinates[2],
            ));
        }
    }

    Ok(data)
}
struct Distance {
    from: usize,
    to: usize,
    dist: u64,
}

struct Solver<'a> {
    boxes: &'a mut [JunctionBox],
    distances: Vec<Distance>,
    circuit: Vec<usize>,
    circuit_size: Vec<usize>,
}

impl<'a> Solver<'a> {
    fn new(boxes: &'a mut [JunctionBox]) -> Self {
        // We need all distances between all boxes, let's compute:
        let mut distances: Vec<Distance> = Vec::new();
        for i in 0..boxes.len() {
            for j in (i + 1)..boxes.len() {
                let euclidian_distance = ((boxes[i].x.abs_diff(boxes[j].x)).pow(2)
                    + (boxes[i].y.abs_diff(boxes[j].y)).pow(2)
                    + (boxes[i].z.abs_diff(boxes[j].z)).pow(2)).isqrt();
                
                distances.push(Distance {
                    from: boxes[i].id,
                    to: boxes[j].id,
                    dist: euclidian_distance,
                });
            }
        }

        // sort them as we want the shortest distances first
        distances.sort_by(|a, b| a.dist.cmp(&b.dist));

        // Let's assign each box to its own circuit to begin with
        // This is some kind of disjoint-set / union-find structure
        let count = boxes.len();
        // each box is its own circuit initially
        let circuit_assignment = (0..count).collect();
        // and obviously each circuit is size 1
        let circuit_size = vec![1; count];

        Solver {
            boxes,
            distances,
            circuit: circuit_assignment,
            circuit_size,
        }
    }

    fn get_circuit(&mut self, box_id: usize) -> usize {
        // pointing to itself means it's the root and hence the circuit id
        if self.circuit[box_id] == box_id {
            return box_id;
        }
        
        // otherwise, recurse to find the circuit id (root)
        let path = self.circuit[box_id];

        // update as we go to be quicker next time (path compression)
        self.circuit[box_id] = self.get_circuit(path);
        
        // now we have it:
        self.circuit[box_id]
    }

    fn connect_junction_boxes(&mut self, first: usize, second: usize) {
        let first_circuit = self.get_circuit(first);
        let second_circuit = self.get_circuit(second);

        if first_circuit != second_circuit {
            // We simply point the first circuit to the second one, plus udpate the size, and they are merged
            self.circuit[first_circuit] = second_circuit;
            self.circuit_size[second_circuit] += self.circuit_size[first_circuit];
        }
    }

    fn part1(&mut self, initial_connection_count: usize) -> u64 {
        // connect the first N boxes with shortest distance
        let connections: Vec<(usize, usize)> = self
            .distances
            .iter()
            .take(initial_connection_count)
            .map(|d| (d.from, d.to))
            .collect();

        for (from, to) in connections {
            self.connect_junction_boxes(from, to);
        }

        // Collect the sizes of all circuits (with size)
        let mut sizes: Vec<u64> = self
            .circuit
            .iter()
            .enumerate()
            .filter_map(|(i, &p)| {
                // it's a "root node" of circuit if it's the same as its pointer
                // we only count those
                if i == p {     
                    Some(self.circuit_size[i] as u64)
                } else {
                    None
                }
            })
            .collect();

        // sort descending
        sizes.sort_by(|a, b| b.cmp(a));

        // product of the top 3
        sizes.iter().take(3).product()
    }

    fn part2(&mut self, initial_connection_count: usize) -> u64 {
        // continue connecting boxes until all are connected, we skip the ones
        // we already connected in part 1...
        let connections: Vec<(usize, usize)> = self
            .distances
            .iter()
            .skip(initial_connection_count)
            .map(|d| (d.from, d.to))
            .collect();

        for (from, to) in connections {
            self.connect_junction_boxes(from, to);
            let circuit = self.get_circuit(from);
            // See if this circuit now includes all boxes, if so, we're done
            if self.circuit_size[circuit] == self.boxes.len() {
                // Answer is the product of the x coordinates of the two boxes that completed the circuit
                return self.boxes[from].x * self.boxes[to].x;
            }
        }

        0
    }
}

#[cfg(test)]
mod tests;
