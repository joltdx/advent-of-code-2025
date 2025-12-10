use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

const BITS: [u16; 10] = [
    0b0000000000000001,
    0b0000000000000010,
    0b0000000000000100,
    0b0000000000001000,
    0b0000000000010000,
    0b0000000000100000,
    0b0000000001000000,
    0b0000000010000000,
    0b0000000100000000,
    0b0000001000000000,
];

fn main() {
    // Get the input filename from command line arguments or default to "input.txt"
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).map(|s| s.as_str()).unwrap_or("input.txt");

    // Read the input file
    match read_input(filename) {
        // If successful, run parts 1 and 2 and measure their execution time
        Ok(contents) => {
            let mut solver = Solver::new(contents);

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

fn read_input(file: &str) -> Result<Vec<Machine>, Box<dyn std::error::Error>> {
    let input_file = File::open(file)?;
    let buffered = BufReader::new(input_file);

    let mut data: Vec<Machine> = Vec::new();

    // Read each line from the file and parse it accordingly
    for line in buffered.lines() {
        let line = line?;
        let mut machine = Machine {
            light_diagram: 0,
            button_wiring: Vec::new(),
            button_bitset: Vec::new(),
            joltage_requirements: Vec::new(),
        };

        for part in line.split_ascii_whitespace() {
            match part.chars().nth(0) {
                Some('[') => {
                    // strip the brackets and get the light bits
                    let lights_str = &part[1..part.len() - 1];
                    for (i, state) in lights_str.chars().enumerate() {
                        if state == '#' {
                            machine.light_diagram |= BITS[i];
                        }
                    }
                }
                Some('(') => {
                    // strip the parentheses and get the button wirings
                    let buttons_str = &part[1..part.len() - 1];
                    let buttons: Vec<u8> = buttons_str
                        .split(',')
                        .map(|s| s.trim().parse().unwrap_or(0))
                        .collect();

                    let mut button_bits: u16 = 0;
                    let mut button_wiring = Vec::new();
                    for button in buttons.iter() {
                        button_bits |= BITS[*button as usize];
                        button_wiring.push(*button);
                    }
                    machine.button_wiring.push(button_wiring);
                    machine.button_bitset.push(button_bits);
                }
                Some('{') => {
                    // strip the curly braces and get the joltage requirements
                    let joltage_str = &part[1..part.len() - 1];
                    let joltage_reqs: Vec<u16> = joltage_str
                        .split(',')
                        .map(|s| s.trim().parse().unwrap_or(0))
                        .collect();
                    
                    for jolt in joltage_reqs.iter() {
                        machine.joltage_requirements.push(*jolt);
                    }
                }
                _ => {}
            }
        }

        data.push(machine);
    }

    Ok(data)
}

struct Machine {
    light_diagram: u16,
    button_wiring: Vec<Vec<u8>>,
    button_bitset: Vec<u16>,
    joltage_requirements: Vec<u16>,
}

struct Solver {
    //machines: &'a [Machine],
    machines: Vec<Machine>,
}

impl Solver {
    fn new(machines: Vec<Machine>) -> Self {
        Solver { machines }
    }

    fn press_for_lights(&self, machine: &Machine) -> u64 {
        // let's store light states and corresponding number of presses in a queue
        let mut queue = VecDeque::new();
        // we store the visited light states (later visits are always more presses)
        // 10 lights max = 10 bits = 1024 possible states
        let mut visited = [false; 1024];

        // First state is of course no lights lite after 0 presses...
        queue.push_back((0b00_0000_0000 as u16, 0 as usize));
        visited[0] = true;

        // Emtpy the queue until we find a matching lights state
        while let Some((lights, presses)) = queue.pop_front() {
            // We good?
            if lights == machine.light_diagram {
                return presses as u64;
            }

            // from this state, we try each button press
            for &button in machine.button_bitset.iter() {
                // press the button
                let new_lights = lights ^ button;

                // If we've seen this state of lights before, it was with
                // fewer or equal presses, so we need not continue with this one
                if visited[new_lights as usize] {
                    continue;
                }

                // But if it's a new combination of lights lit, enqueue it to dig deeper
                visited[new_lights as usize] = true;
                queue.push_back((new_lights, presses + 1));
            }
        }

        // if we're here, we're screwed
        0
    }

    fn press_for_joltage(&self, machine: &Machine) -> u64 {
        // We need to go for math here, solving a system of linear equations
        // using Gaussian elimination with back substitution, at least it started 
        // like that. Luckily there today is google and AI nowadays to help with that... :D

        // This finally working solution was proudly iterated into existence
        // by me trying stuff, having Geimini suggest and refactor things, 
        // optimizing stuff, rinse and repeat until it was first of all correct,
        // and then also fast enough.
        // We want to minimize the total number of button presses, so we will
        // treat free variables (if any) accordingly.
        let num_vars = machine.button_wiring.len();
        let num_eqs = machine.joltage_requirements.len();

        // We're solving Ax = B, where A is the button wiring matrix,
        // B is the joltage requirements, and x is the number of presses for each button
        
        // Build augmented matrix [A | B]
        let mut matrix = vec![vec![0.0; num_vars + 1]; num_eqs];
        for (j, wiring) in machine.button_wiring.iter().enumerate() {
            for &counter_idx in wiring {
                if (counter_idx as usize) < num_eqs {
                    matrix[counter_idx as usize][j] = 1.0;
                }
            }
        }
        for i in 0..num_eqs {
            matrix[i][num_vars] = machine.joltage_requirements[i] as f64;
        }

        // Gaussian Elimination time!
        // We want to have some variable as a single unknown in each equation if possible
        // Work this into a "Row Echelon Form", so we at least have that on the last line
        let mut pivot_row = 0;
        let mut col_to_pivot_row = vec![None; num_vars];
        let mut free_cols = Vec::new();

        for col in 0..num_vars {
            if pivot_row >= num_eqs {
                // Ran out of rows, so this column is free
                free_cols.push(col);
                continue;
            }

            // Find the best pivot row for this column (partial pivoting)
            // Basically find the row with the largest absolute value in this column
            // to avoid numerical instability (though with integers it's less of an issue, but still good practice)
            let mut max_row = pivot_row;
            for row in pivot_row + 1..num_eqs {
                if matrix[row][col].abs() > matrix[max_row][col].abs() {
                    max_row = row;
                }
            }

            // If the pivot is essentially zero, we can't use it
            if matrix[max_row][col].abs() < 1e-9 {
                free_cols.push(col);
                continue;
            }

            // We want to be at the pivot row, so we swap the current for it...
            matrix.swap(pivot_row, max_row);

            // Normalize the pivot row so the pivot element becomes 1
            let pivot_val = matrix[pivot_row][col];
            for c in col..=num_vars {
                matrix[pivot_row][c] /= pivot_val;
            }

            // Eliminate other rows to make everything below the pivot zero
            for row in 0..num_eqs {
                if row != pivot_row {
                    let factor = matrix[row][col];
                    for c in col..=num_vars {
                        matrix[row][c] -= factor * matrix[pivot_row][c];
                    }
                }
            }

            // Remember which row is the pivot for this column
            col_to_pivot_row[col] = Some(pivot_row);
            pivot_row += 1;
        }

        // Check for inconsistency (e.g. 0 = 5)
        // If we have a row of zeros equal to something non-zero, it's impossible
        for row in pivot_row..num_eqs {
            if matrix[row][num_vars].abs() > 1e-9 {
                return 0; // Impossible
            }
        }

        // Solve for free variables
        let mut min_total_presses = u64::MAX;
        
        // Precompute weights for free variables
        // W_j = 1 - sum(matrix[row][col]) for pivot rows
        // This represents how much the total press count changes when we increase free var j by 1
        let mut free_var_weights = Vec::new();
        for &col in &free_cols {
            let mut weight = 1.0;
            for row in 0..num_eqs {
                // If this row is a pivot row, it depends on the free variable
                // x_pivot = V - ... - A_rj * x_j
                // So x_pivot changes by -A_rj
                if matrix[row][num_vars].abs() > 1e-9 || (0..num_vars).any(|c| matrix[row][c].abs() > 1e-9) {
                     weight -= matrix[row][col];
                }
            }
            free_var_weights.push(weight);
        }

        // Base cost is the sum of the constants in the pivot equations
        // (Assuming all free vars are 0)
        let mut base_cost = 0.0;
        for row in 0..num_eqs {
             // Only count if it's a valid equation row (not 0=0)
             if matrix[row][num_vars].abs() > 1e-9 || (0..num_vars).any(|c| matrix[row][c].abs() > 1e-9) {
                base_cost += matrix[row][num_vars];
             }
        }

        // Stack for DFS: (idx, current_solution, current_cost, current_rhs)
        // current_rhs[r] stores (V_r - sum(A_rj * x_j)) for assigned j
        let initial_rhs: Vec<f64> = (0..num_eqs).map(|r| matrix[r][num_vars]).collect();
        
        let mut stack = Vec::new();
        stack.push((0, vec![0i64; num_vars], base_cost, initial_rhs)); 

        while let Some((idx, mut solution, current_cost, current_rhs)) = stack.pop() {
            // Pruning 1: Cost Lower Bound
            // Calculate minimum possible cost from unassigned variables
            let mut min_future_cost = 0.0;
            for i in idx..free_cols.len() {
                if free_var_weights[i] < 0.0 {
                    min_future_cost += free_var_weights[i] * 300.0;
                }
            }
            if current_cost + min_future_cost >= (min_total_presses as f64) - 1e-9 {
                continue;
            }

            // Pruning 2: Feasibility Lookahead
            // For each row, check if it's possible to keep the pivot non-negative
            // RHS_r >= sum(A_rk * x_k) for unassigned k
            // We need max possible sum(A_rk * x_k) <= RHS_r ?? No.
            // We need x_pivot = RHS_r - sum(...) >= 0
            // So sum(...) <= RHS_r
            // So we need min(sum(...)) <= RHS_r. If min(sum) > RHS, then impossible.
            let mut possible = true;
            for row in 0..num_eqs {
                // Only check rows that are actually pivot rows (or relevant constraints)
                // Actually we can just check all rows, 0=0 rows won't matter
                let mut min_future_subtraction = 0.0;
                for i in idx..free_cols.len() {
                    let col = free_cols[i];
                    let coeff = matrix[row][col];
                    // We subtract coeff * x. We want to minimize the subtraction (maximize the result)
                    // to see if it's even possible to stay >= 0.
                    // Actually, we want to know if the *minimum* subtraction is already too much.
                    // The term in x_pivot is -coeff * x.
                    // So we add -coeff * x.
                    // We want to maximize (-coeff * x).
                    // If max(-coeff * x) + RHS < 0, then impossible.
                    
                    // Equivalently: x_pivot = RHS - (coeff * x).
                    // We need RHS - (coeff * x) >= 0.
                    // RHS >= coeff * x.
                    // We need to find if there exists ANY assignment of future x such that this holds.
                    // So we need RHS >= min(coeff * x).
                    
                    if coeff > 0.0 {
                        // min(coeff * x) is when x=0 -> 0
                        min_future_subtraction += 0.0;
                    } else {
                        // coeff is negative. min(coeff * x) is when x=300 -> coeff * 300
                        min_future_subtraction += coeff * 300.0;
                    }
                }
                
                if current_rhs[row] < min_future_subtraction - 1e-9 {
                    possible = false;
                    break;
                }
            }
            if !possible {
                continue;
            }

            // If we've assigned values to all free variables...
            if idx == free_cols.len() {
                // We already have the cost estimate, but let's verify integer constraints on pivots
                let mut valid = true;
                let mut final_total = 0;
                
                // Sum free vars
                for &col in &free_cols {
                    final_total += solution[col];
                }

                // Check pivots
                for col in 0..num_vars {
                    if let Some(row) = col_to_pivot_row[col] {
                        let val = current_rhs[row]; // This is fully computed now

                        if val < -1e-9 {
                            valid = false;
                            break;
                        }
                        let rounded = val.round();
                        if (val - rounded).abs() > 1e-9 {
                            valid = false;
                            break;
                        }

                        let int_val = rounded as i64;
                        solution[col] = int_val;
                        final_total += int_val;
                    }
                }

                if valid {
                    if (final_total as u64) < min_total_presses {
                        min_total_presses = final_total as u64;
                    }
                }
                continue;
            }

            let col = free_cols[idx];
            let weight = free_var_weights[idx];
            
            // Heuristic: If weight is positive, try small values first.
            // If weight is negative, try large values first.
            // This helps find a good min_total_presses early, making pruning more effective.
            if weight >= 0.0 {
                for val in 0..=300 {
                    let mut new_sol = solution.clone();
                    new_sol[col] = val;
                    
                    let mut new_rhs = current_rhs.clone();
                    for row in 0..num_eqs {
                        new_rhs[row] -= matrix[row][col] * (val as f64);
                    }
                    
                    let new_cost = current_cost + weight * (val as f64);
                    stack.push((idx + 1, new_sol, new_cost, new_rhs));
                }
            } else {
                for val in (0..=300).rev() {
                    let mut new_sol = solution.clone();
                    new_sol[col] = val;
                    
                    let mut new_rhs = current_rhs.clone();
                    for row in 0..num_eqs {
                        new_rhs[row] -= matrix[row][col] * (val as f64);
                    }
                    
                    let new_cost = current_cost + weight * (val as f64);
                    stack.push((idx + 1, new_sol, new_cost, new_rhs));
                }
            }
        }

        if min_total_presses == u64::MAX {
            0
        } else {
            min_total_presses
        }
    }

    fn part1(&mut self) -> u64 {
        let mut sum: u64 = 0;

        for machine in self.machines.iter() {
            sum += self.press_for_lights(machine);
        }

        sum
    }

    fn part2(&self) -> u64 {
        let mut sum: u64 = 0;

        for machine in self.machines.iter() {
            sum += self.press_for_joltage(machine);
        }
        
        sum
    }
}

#[cfg(test)]
mod tests;
