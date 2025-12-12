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
        Ok((shapes, regions)) => {
            let solver = Solver::new(&shapes, &regions);

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

fn read_input(file: &str) -> Result<(Vec<Shape>, Vec<Region>), Box<dyn std::error::Error>> {
    let input_file = File::open(file)?;
    let mut buffered = BufReader::new(input_file);

    let mut shapes: Vec<Shape> = Vec::new();
    let mut regions: Vec<Region> = Vec::new();

    // Read each line from the file and parse it accordingly
    // First get the 6 shapes:
    for _ in 0..6 {
        let mut dont_bother_with_this_line = String::new();
        let mut shape_lines = String::new();

        // read the index line before the shape
        buffered.read_line(&mut dont_bother_with_this_line)?; 
        
        // read the 3 lines of the shape, parse it and store it
        for _ in 0..3 {
            let mut line = String::new();
            buffered.read_line(&mut line)?;
            shape_lines.push_str(&line);
        }
        shapes.push(Shape::from_str(&shape_lines));

        // Read the empty line between shapes
        buffered.read_line(&mut dont_bother_with_this_line)?;
    }
    
    // Next up we read the region definitions
    for line in buffered.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 2 {
            return Err(format!("Invalid region definition: {}", line).into());
        }
        let dimensions: Vec<&str> = parts[0].trim().split('x').collect();
        if dimensions.len() != 2 {
            return Err(format!("Invalid region dimensions: {}", parts[0]).into());
        }
        let width: usize = dimensions[0].trim().parse()?;
        let height: usize = dimensions[1].trim().parse()?;
        let shape_counts: Vec<usize> = parts[1]
            .trim()
            .split_whitespace()
            .map(|s| s.parse())
            .collect::<Result<Vec<usize>, _>>()?;
        if shape_counts.len() != 6 {
            return Err(format!("Invalid shape counts: {}", parts[1]).into());
        }
        let region = Region {
            width,
            height,
            shape_quantity: [
                shape_counts[0],
                shape_counts[1],
                shape_counts[2],
                shape_counts[3],
                shape_counts[4],
                shape_counts[5],
            ],
        };
        regions.push(region);
    }

    Ok((shapes, regions))
}

struct Region {
    width: usize,
    height: usize,
    shape_quantity: [usize; 6],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Shape {
    rows: [u64; 3], // bit pattern for each row of 3x3 box
                    // (we use u64 to be able to shift bits into the u64 width grid...)
    size: usize,    // number of # in the box
}

impl Shape {
    // parse a shape from a 3-line string
    fn from_str(s: &str) -> Self {
        let mut rows = [0u64; 3];
        let mut size = 0;

        for (y, line) in s.lines().enumerate().take(3) {
            for (x, char) in line.chars().enumerate().take(3) {
                if char == '#' {
                    rows[y] |= 1 << x;  // set the bit for the #
                    size += 1;          // increment size
                }
            }
        }

        Shape { rows, size }
    }

    // wee need to rotate the box 90 degrees clockwise
    fn rotate_box(&self) -> Shape {
        let mut new_rows = [0u64; 3];

        for y in 0..3 {
            for x in 0..3 {
                if (self.rows[y] & (1 << x)) != 0 {
                    new_rows[x] |= 1 << (2 - y);
                }
            }
        }

        Shape {
            rows: new_rows,
            size: self.size,
        }
    }

    // we also need to be able to flip the box horizontally
    fn flip_box(&self) -> Shape {
        let mut new_rows = [0u64; 3];

        for y in 0..3 {
            for x in 0..3 {
                if (self.rows[y] & (1 << x)) != 0 {
                    new_rows[y] |= 1 << (2 - x);
                }
            }
        }

        Shape {
            rows: new_rows,
            size: self.size,
        }
    }

    // let's generate all variants of this shape (4 rotations, each flipped and unflipped))
    fn generate_variants(&self) -> Vec<Shape> {
        let mut variants = Vec::new();
        let mut current = *self;

        for _ in 0..4 {
            variants.push(current);
            variants.push(current.flip_box());
            current = current.rotate_box();
        }

        // Get rid of any duplicates
        variants.sort_by_key(|b| b.rows);
        variants.dedup_by_key(|b| b.rows);

        variants
    }
}

struct Grid {
    rows: Vec<u64>, // u64 is wide enough for our purposes here, each bit represents a cell in the grid
    width: usize,   // actual width of the grid
    height: usize,  // actual height of the grid
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        Grid {
            rows: vec![0u64; height],
            width,
            height,
        }
    }

    // can the shape fit at this position?
    fn can_fit(&self, shape: &Shape, x: usize, y: usize) -> bool {
        // is there room left at all?
        if x + 3 > self.width || y + 3 > self.height {
            return false;
        }

        for row in 0..3 {
            // shift the shape row to the correct x position
            let shape_row_bits = shape.rows[row] << x;
            if (self.rows[y + row] & shape_row_bits) != 0 {
                return false;   // overlap not allowed
            }
        }
        true
    }

    // let's put the shape here
    fn place_shape(&mut self, shape: &Shape, x: usize, y: usize) {
        for row in 0..3 {
            let shape_row_bits = shape.rows[row] << x;
            self.rows[y + row] |= shape_row_bits;
        }
    }

    // remove the shape from here
    fn remove_shape(&mut self, shape: &Shape, x: usize, y: usize) {
        for row in 0..3 {
            let shape_row_bits = shape.rows[row] << x;
            self.rows[y + row] &= !shape_row_bits;
        }
    }

}

struct Solver<'a> {
    _shapes: &'a [Shape],
    regions: &'a [Region],
    shape_variants: Vec<Vec<Shape>>,
}

impl<'a> Solver<'a> {
    fn new(shapes: &'a [Shape], regions: &'a [Region]) -> Self {
        let mut shape_variants = Vec::new();
        for shape in shapes {
            shape_variants.push(shape.generate_variants());
        }
        Solver { _shapes: shapes, regions, shape_variants }
    }

    fn fit_the_presents(&self, grid: &mut Grid, presents: &[&Vec<Shape>], index: usize) -> bool {
        if index == presents.len() {
            return true; // all shapes placed, wohoo \o/
        }

        let shape_variants = presents[index];

        // try to place this present in all positions and variations (well, one at a time,
        // and only backtrack if needed)
        // this is a bit too much because when we have presents placed already, we will always
        // have occupied cells in the grid, so we could optimize a bit by skipping those positions
        // but for now, let's keep it simple 
        for y in 0..grid.height {
            for x in 0..grid.width {
                for shape in shape_variants {
                    if grid.can_fit(shape, x, y) {
                        // if it fits, it sits...
                        grid.place_shape(shape, x, y);
                        // try to fit the next present
                        if self.fit_the_presents(grid, presents, index + 1) {
                            return true;    // All good, we done!
                        }
                        // so we're back here because the shape didn't fit, we backtrack and try something else
                        grid.remove_shape(shape, x, y);
                    }
                }
            }
        }

        false // no valid placement found
    }

    fn part1(&self) -> u64 {
        let mut count: u64 = 0;
        let mut count_oversized = 0;
        let mut count_no_solution = 0;

        // Check each region one by one and count how many can fit the presents
        for region in self.regions {
            let mut total_present_size = 0;
            let mut present_list = Vec::new();
            for (i, &count) in region.shape_quantity.iter().enumerate() {
                for _ in 0..count {
                    present_list.push(&self.shape_variants[i]);
                    total_present_size += self.shape_variants[i][0].size;
                }
            }

            // Leave early if there is not enought space at all
            if total_present_size > region.width * region.height {
                count_oversized += 1;
                continue;
            }

            // sort the shape list by area descending for hopefully a bit more luck with the placement...
            present_list.sort_by_key(|s| usize::MAX - s[0].size); 


            let mut grid = Grid::new(region.width, region.height);
            if self.fit_the_presents(&mut grid, &present_list, 0) { 
                count += 1;
            } else {
                count_no_solution += 1;
            }
        }

        println!("Number of regions: {}", self.regions.len());
        println!("Number of regions with a solution: {}", count);
        println!("Number of oversized regions: {}", count_oversized);
        println!("Number of regions with no solution: {}", count_no_solution);
        println!("What in the wat! It would've been enough with just the oversize check... o_O\n");

        count
    }

    fn part2(&self) -> &str {
        "There is no part 2"
    }
}

#[cfg(test)]
mod tests;