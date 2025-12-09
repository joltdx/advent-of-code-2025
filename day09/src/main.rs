use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::collections::HashSet;

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

fn read_input(file: &str) -> Result<Vec<(u64, u64)>, Box<dyn std::error::Error>> {
    // For just reading into a vector of strings, this function could just be:
    //    let content = fs::read_to_string(file)?;
    //    Ok(content.lines().map(String::from).collect())
    // but most day'es well be parsing data here, line by line, so let's have
    // the buffered reading here...
    let input_file = File::open(file)?;
    let buffered = BufReader::new(input_file);

    let mut data: Vec<(u64, u64)> = Vec::new();

    // Read each line from the file and parse it accordingly
    for line in buffered.lines() {
        let line = line?;
        let coordinates: Vec<&str> = line.split(',').collect();
        if coordinates.len() == 2 {
            let x = coordinates[0].trim().parse::<u64>()?;
            let y = coordinates[1].trim().parse::<u64>()?;
            data.push((x, y));
        }
    }

    Ok(data)
}

struct Solver<'a> {
    red_tiles: &'a [(u64, u64)],
}

impl<'a> Solver<'a> {
    fn new(red_tiles: &'a [(u64, u64)]) -> Self {
        Solver { red_tiles }
    }

    fn part1(&self) -> u64 {
        let mut max_area: u64 = 0;

        for i in 0..self.red_tiles.len() {
            for j in (i + 1)..self.red_tiles.len() {
                let first = self.red_tiles[i];
                let second = self.red_tiles[j];
                let area = (first.0.abs_diff(second.0) + 1) * (first.1.abs_diff(second.1) + 1);
                if area > max_area {
                    max_area = area;
                }
            }
        }        

        max_area
    }

   fn _abandoned_part2(&self) -> u64 {
        // This was waaaay too slow, trying to check each and every point in the rectanble...
        let mut max_area: u64 = 0;

        let mut vertical_edges: HashSet<(u32,u32)> = HashSet::new();
        let mut prev = self.red_tiles.first().unwrap();
        for coord in self.red_tiles.iter().skip(1) {
            if coord.0 == prev.0 {
                // vertical edge
                let y_start = prev.1.min(coord.1) as u32;
                let y_end = prev.1.max(coord.1) as u32;
                for y in y_start..=y_end {
                    vertical_edges.insert((coord.0 as u32, y));
                }
            }
            prev = coord;
        }

        for i in 0..self.red_tiles.len() {
            for j in (i + 1)..self.red_tiles.len() {
                let first = self.red_tiles[i];
                let second = self.red_tiles[j];
                let area = (first.0.abs_diff(second.0) + 1) * (first.1.abs_diff(second.1) + 1);
                if area > max_area {
                    // Are we contained by vertical edges?
                    let x_start = first.0.min(second.0) as u32;
                    let x_end = first.0.max(second.0) as u32;
                    let y_start = first.1.min(second.1) as u32;
                    let y_end = first.1.max(second.1) as u32;
                    let mut contained = true;
                    for x in x_start..=x_end {
                        for y in y_start..=y_end {
                            let mut crossings = 0;
                            for ray in 0..=x {
                                if vertical_edges.contains(&(ray, y)) {
                                    crossings += 1;
                                }
                            }
                            if crossings % 2 == 0 {
                                if !vertical_edges.contains(&(x, y)) {
                                    contained = false;
                                    break;
                                }
                            }
                        }
                        if !contained {
                            break;
                        }
                    }
                    if contained {
                        max_area = area;
                    }

                }
            }
        }

        max_area
    }

    fn part2(&self) -> u64 {
        let mut max_area: u64 = 0;

        let num_red_tiles = self.red_tiles.len();

        // First (abandoned) solution tried to verify each point in the rectangle as being inside the crazy
        // tile pattern the elves have, which was way too slow.
        // As the tile pattern is not a crazy polygon thing, we get away with checking only the edges, and
        // one point inside of it, to determine if it's either fully inside or fully outside...

        // Find all of the edges, horizontal and vertical by themselves
        let mut vertical_lines = Vec::new();
        let mut horizontal_lines = Vec::new();
        for i in 0..num_red_tiles {
            // lines are from one tile to the next, wrapping around for the last one using modulo here
            let first = self.red_tiles[i];
            let second = self.red_tiles[(i + 1) % num_red_tiles];

            if first.0 == second.0 {
                // vertical line
                let y_min = first.1.min(second.1);
                let y_max = first.1.max(second.1);
                vertical_lines.push((first.0, y_min, y_max));
            } else if first.1 == second.1 {
                // horizontal line
                let x_min = first.0.min(second.0);
                let x_max = first.0.max(second.0);
                horizontal_lines.push((first.1, x_min, x_max));
            }
        }

        // still loop over all combinations of red tile corners that form a rectangle
        for i in 0..num_red_tiles {
            for j in (i + 1)..num_red_tiles {
                let first = self.red_tiles[i];
                let second = self.red_tiles[j];

                // lines are valid as rectanbles, but will not be the biggest one anyway
                if first.0 == second.0 || first.1 == second.1 {
                    continue;
                }

                let x_start = first.0.min(second.0);
                let x_end = first.0.max(second.0);
                let y_start = first.1.min(second.1);
                let y_end = first.1.max(second.1);

                let area = (x_end - x_start + 1) * (y_end - y_start + 1);
                
                if area <= max_area {
                    continue;
                }

                let mut intersects = false;

                // right, let's see if any edges intersect here, vertical first
                for &(vx, vy_min, vy_max) in &vertical_lines {
                    // if vx is between x_start and x_end, then we might have an intersection
                    if vx > x_start && vx < x_end {
                        // ...but only if we are at also are touching in the y
                        if y_start < vy_max && y_end > vy_min {
                            intersects = true;
                            break;  
                        }
                    }
                }

                if intersects { continue; }

                // ok, check also horizontal edges
                for &(hy, hx_min, hx_max) in &horizontal_lines {
                    // if hy is between y_start and y_end, then we might have an intersection
                    if hy > y_start && hy < y_end {
                        //...but only if we are also touching in the x
                        if x_start < hx_max && x_end > hx_min {
                            intersects = true;
                            break;  
                        }
                    }
                }

                if intersects { continue; }

                // cool, no edges intersect, but are we inside or outside?
                // we pick a test point just inside the rectangle, and do some kind of inverse ray-case thing
                // from the points perspective and to the left until the edge of the map, using the vertical lines
                let test_x = x_start + 1;
                let test_y = y_start + 1;
                let mut crossings = 0;

                for &(vx, vy_min, vy_max) in &vertical_lines {
                    if (vx) < test_x {
                        // we do intersect on x, but we might not touch on the y...
                        if test_y < vy_max && test_y > vy_min {
                            crossings += 1;
                        }
                    }
                }

                // Odd number of crossings means we're inside and we DO have a new max area
                if crossings % 2 == 1 {
                    max_area = area;
                }
            }
        }

        max_area
    }
}

#[cfg(test)]
mod tests;