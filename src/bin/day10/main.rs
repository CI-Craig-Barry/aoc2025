use std::cmp::{max, min, Reverse};
use aoc2025::utils;
use std::collections::{HashMap, HashSet};
use std::ops::Add;
use good_lp::{default_solver, variable, variables, Expression, Solution, SolverModel, Variable};
use priority_queue::PriorityQueue;

pub fn get_details() -> utils::ExecDetails {
    return utils::ExecDetails {
        day: 10,
        sample: include_str!("sample").to_string(),
        sample2: None,
        input: include_str!("input").to_string(),
        task1_function: task1,
        task2_function: task2,
        task1_sample_expected: 7,
        task1_input_expected: 461,
        task2_sample_expected: 33,
        task2_input_expected: 16386,
};
}

fn main() {
    utils::exec(&get_details());
}

#[derive(Clone, Debug)]
struct Button {
    wiring_mask: u16,
    num_bits: u8
}

#[derive(Debug)]
struct Machine {
    state: u16,
    active_state: u16,
    buttons: Vec<Button>,
    joltages: Vec<u16>,
}

fn set_bit(input: u16, index: u8) -> u16 {
    let mask = 1 << index;
    return input | mask;
}

impl Button {
    fn get_button_wirings(&self) -> Vec<u8> {
        let mut button_wirings = Vec::new();

        let lsb_mask = 1;
        let mut wirings_mask = self.wiring_mask;
        for i in 0..self.num_bits {
            if(wirings_mask & lsb_mask == 1)
            {
                button_wirings.push(i);
            }
            wirings_mask >>= 1;
        }

        return button_wirings;
    }
}

impl Machine {
    fn is_active_state(&self) -> bool {
        return self.state == self.active_state;
    }
}

fn parse_machines(file_input: &String, need_joltages: bool) -> Vec<Machine> {
    return file_input.lines().map(|line| {
        // let mut active_state: Vec<bool> = Vec::new();
        let mut active_state: u16 = 0;
        let mut buttons : Vec<Button> = Vec::new();
        let mut joltages: Vec<u16> = Vec::new();

        let mut iter_index = 1;
        let line_bytes = line.as_bytes();

        let mut num_bits = 0;
        //Parse active signals
        loop {
            let char = line_bytes[iter_index];
            iter_index += 1;
            if char == b'#' {
                num_bits += 1;
                active_state >>= 1;
                active_state |= 1 << 15;
            }
            else if char == b'.' {
                num_bits += 1;
                active_state >>= 1;
            }
            else {
                active_state >>= 16 - num_bits;
                break;
            }
        }

        //Parse buttons
        loop {
            let char = line_bytes[iter_index];
            let mut num_bits = 0;
            iter_index += 1;

            if char == b'{'
            {
                break;
            }
            else if char == b'('
            {
                let mut wiring_mask: u16 = 0;
                loop {
                    let char = line_bytes[iter_index];
                    iter_index += 1;

                    if char == b')' {
                        buttons.push(Button {
                            wiring_mask,
                            num_bits
                        });
                        break;
                    }
                    else if char.is_ascii_digit() {
                        let indicator_index: u8 = char - b'0';
                        wiring_mask = set_bit(wiring_mask, indicator_index);
                        num_bits = indicator_index + 1;
                    }
                }
            }
        }

        if need_joltages
        {
            let mut buffer = String::new();
            loop {
                let char = line_bytes[iter_index];
                iter_index += 1;

                if char.is_ascii_digit() {
                    buffer.push(char as char);
                }
                else if char == b',' {
                    joltages.push(buffer.parse::<u16>().unwrap());
                    buffer.clear();
                }
                else {
                    joltages.push(buffer.parse::<u16>().unwrap());
                    buffer.clear();
                    break;
                }
            }
        }

        return Machine {
            state: 0,
            active_state,
            buttons,
            joltages,
        };
    }).collect::<Vec<Machine>>();
}

fn recur_find_num_button_presses(
    machine: &mut Machine,
    cur_attempt_number: u32,
    start_states: &HashSet<u16>
) -> u32
{
    let mut new_states: HashSet<u16> = HashSet::new();
    let this_attempt_number: u32 = cur_attempt_number + 1;

    for button in &machine.buttons {
        for start_state in start_states {
            machine.state = *start_state;
            machine.state ^= button.wiring_mask;

            if machine.is_active_state() {
                return this_attempt_number;
            }
            new_states.insert(machine.state);
        }
    }

    return recur_find_num_button_presses(machine, this_attempt_number, &new_states);
}

pub fn task1(file_input: &String) -> i64 {
    let machines = parse_machines(file_input, false);

    let mut total = 0;
    let mut start_set = HashSet::new();
    start_set.insert(0);
    for mut machine in machines {
        let num_presses = recur_find_num_button_presses(&mut machine, 0, &start_set);
        total += num_presses;
    }

    return total as i64;
}

fn find_max_presses(machine: &Machine) -> Vec<u16>
{
    let mut max_presses : Vec<u16> = Vec::new();

    for button in &machine.buttons {
        let max_button_presses = button.get_button_wirings()
          .iter()
          .map(|wiring| {
              machine.joltages[*wiring as usize]
          })
          .min();

        max_presses.push(max_button_presses.unwrap() as u16);
    }

    return max_presses;
}

fn find_possible_btn_presses_remaining(max_presses: &Vec<u16>) -> Vec<u16>
{
    let mut possible_btn_presses : Vec<u16> = Vec::new();
    let mut total = 0;

    for max_presses in max_presses.iter().rev() {
        possible_btn_presses.insert(0, total);
        total += *max_presses;
    }

    return possible_btn_presses;
}

fn recur_possible_sequences(
    machine: &Machine,
    cur_joltages: Vec<u16>,
    remaining_iterations: u16,
    button_index: u8,
    max_button_presses: &Vec<u16>,
    remaining_button_presses: &Vec<u16>,
) -> bool
{
    let max_button_press = max_button_presses[button_index as usize];
    let remaining_button_press = remaining_button_presses[button_index as usize];
    let min_num_iterations = max(remaining_iterations as i16 - remaining_button_press as i16, 0) as u16;
    let max_num_iterations = min(remaining_iterations, max_button_press) + 1;
    let button = &machine.buttons[button_index as usize];

    let is_last_button = button_index as usize == max_button_presses.len() - 1;

    for i in min_num_iterations..max_num_iterations {
        let mut new_joltages = cur_joltages.clone();
        for wiring in button.get_button_wirings() {
            new_joltages[wiring as usize] += i;
        }

        if !is_last_button {
            let result = recur_possible_sequences(
                machine,
                new_joltages,
                remaining_iterations - i,
                button_index + 1,
                max_button_presses,
                remaining_button_presses
            );

            if(result)
            {
                return true;
            }
        }
        else {
            let correct_joltages = &machine.joltages;
            return new_joltages == *correct_joltages;
        }
    }

    return false;
}

fn task2_attempt1(file_input: &String) -> i64 {
    let machines = parse_machines(file_input, true);
    let mut total: u64 = 0;

    for (index, machine) in machines.iter().enumerate() {
        let max_button_presses = find_max_presses(&machine);
        let remaining_button_presses = find_possible_btn_presses_remaining(&max_button_presses);
        let min_total_presses: u16 = *machine.joltages.iter().max().unwrap() as u16;

        let mut current_iterations = min_total_presses;
        loop {
            let result = recur_possible_sequences(
                &machine,
                vec!(0; machine.joltages.len()),
                current_iterations,
                0,
                &max_button_presses,
                &remaining_button_presses
            );

            if result
            {
                total += current_iterations as u64;
                println!("Complete {}/{}", index+1, machines.len());
                break;
            }

            current_iterations += 1;
        }
    }

    return total as i64;
}

type Point = Vec<u16>;

fn heuristic(current: &Point, goal: &Point) -> u32 {
    // let mut h = 0;
    //
    // for i in 0..goal.len() {
    //     let diff = goal[i] as i32 - current[i] as i32;
    //     // h += diff as u32;
    //     h = max(h, diff as u32);
    // }
    //
    // return h;

    return 0;
}

fn get_neighbours(
    machine: &Machine,
    current: &Point,
    goal: &Point,
) -> Vec<(u16, Point)>
{
    let mut results = Vec::new();

    for button in &machine.buttons {
        let mut num_presses = 1;

        loop {
            let mut neighbour_pt: Point = current.clone();
            let mut neighbour_valid = true;
            let mut g_cost = 0;

            for wiring in button.get_button_wirings() {
                neighbour_pt[wiring as usize] += num_presses;
                g_cost += num_presses;

                if neighbour_pt[wiring as usize] > goal[wiring as usize]
                {
                    neighbour_valid = false;
                    break;
                }
            }

            if !neighbour_valid {
                break;
            }

            results.push((g_cost, neighbour_pt));
            num_presses = num_presses + 1;
        }
    }

    return results;
}

fn astar(
    machine: &Machine,
    start: &Point,
    goal: &Point
) -> u32
{
    let mut open_set = PriorityQueue::new();
    let mut closed_set: HashSet<Point> = HashSet::new();
    let mut g_scores: HashMap<Point, u32> = HashMap::new();
    let mut came_from: HashMap<Point, Point> = HashMap::new();

    let initial_h = heuristic(start, goal);
    open_set.push(start.clone(), Reverse(initial_h));
    g_scores.insert(start.clone(), 0);

    while !open_set.is_empty() {
        let current = open_set.pop().unwrap().0;

        // println!("{:?}", open_set.clone().into_iter().collect::<Vec<_>>());
        if current == *goal {
            //TODO - Reconstruct path? Don't actually need to
            let g_score = g_scores[&current];

            let mut c = Some(goal);
            while c.is_some() {
                // println!("came from {:?} {}", c.unwrap(), g_scores[c.unwrap()]);
                c = came_from.get(c.unwrap())
            }

            return g_score;
        }

        closed_set.insert(current.clone());

        let neighbours = get_neighbours(machine, &current, &goal);

        //Find neighbours of the current state
        for neighbour in &neighbours {
            let g_cost = neighbour.0;
            let neighbour_pt = &neighbour.1;

            //If neighbour is not a valid neighbour or is in the closed set, we skip it
            if(closed_set.contains(neighbour_pt))
            {
                // println!("SKIP {}, {}", !neighbour_valid, closed_set.contains(&neighbour_pt));
                continue;
            }

            //TODO - Could improve this by generating neighbours as n presses of button?
            //G score is previous score + 1 as each neighbour requires 1 iteration
            let tentative_g = g_scores.get(&current).unwrap() + g_cost as u32;
            let h_cost = heuristic(&neighbour_pt, goal);
            let f_cost = tentative_g + h_cost;

            let open_set_neighbour = open_set.get(neighbour_pt);
            //If we have not yet explored this neighbour, add it to the open set
            if open_set_neighbour.is_none() {
                open_set.push(neighbour_pt.clone(), Reverse(f_cost));
            }
            // This is worse than current method to this neighbour, ignore it
            else if(tentative_g >= g_scores[open_set_neighbour.unwrap().0]) {
                continue
            }

            // We found a better solution, override current solution
            came_from.insert(neighbour_pt.clone(), current.clone());
            let old_opt = open_set.change_priority(neighbour_pt, Reverse(f_cost));
            if old_opt.is_none()
            {
                println!("FUCKIN PROBLEM HOMIE");
            }
            g_scores.insert(neighbour_pt.clone(), tentative_g);
        }

    }

    return u32::MAX;
}

fn task2_attempt2(file_input: &String) -> i64 {
    let machines = parse_machines(file_input, true);
    let mut total = 0;
    for (index, machine) in machines.iter().enumerate() {
        let cost = astar(machine, &vec![0; machine.joltages.len()], &machine.joltages);
        total += cost;

        println!("Complete {}/{}", index+1, machines.len());
    }

    return total as i64;
}

fn gaussian_elimination(
    A: Vec<Vec<i16>>,
    height: usize,
    width: usize,
) -> Vec<Vec<i16>>
{
    let mut matrix = A.clone();
    let mut pivot_row: usize = 0; // pivot row
    let mut pivot_col: usize = 0; // pivot column

    while (pivot_row < height &&  pivot_col < width) {

        //Find the k-th pivot (i.e. the pivot in the column)
        //(i.e. the index of the pivot)
        let mut idx_max: usize = 0;
        let mut val_max: i16 = 0;
        for i in pivot_row..height {
            let value = (matrix[i as usize][pivot_col as usize]).abs() as i16;
            if(value > val_max)
            {
                val_max = value;
                idx_max = i;
            }
        }

        //If idx_max == 0 then there is no pivot in this column
        if(val_max == 0) {
            pivot_col += 1;
            continue
        }

        //If we reached here we have identified a pivot in the column
        //Swap pivot row with i_max
        let temp1 = matrix[pivot_row].clone();
        let temp2 = matrix[idx_max].clone();
        matrix[idx_max] = temp1;
        matrix[pivot_row] = temp2;

        //Iterate all rows before the pivot
        for i in pivot_row +1..height {
            let f = matrix[i][pivot_col] / matrix[pivot_row][pivot_col];

            //Fill lower part of pivot column with zeros
            matrix[i][pivot_col] = 0;

            //For rest of elements in the current row
            for j in pivot_col +1..width {
                matrix[i][j] = matrix[i][j] - matrix[pivot_row][j] * f;
            }
        }

        //Increment pivots
        pivot_row += 1;
        pivot_col += 1;
    }

    return matrix;
}

fn gauss_jordan_elimination(
    mut matrix: Vec<Vec<i16>>,
    height: usize,
    width: usize
) -> Vec<Vec<i16>> {
    let rows = height;
    let cols = width;

    let mut pivot_row = 0;
    let mut pivot_col = 0;

    while pivot_row < rows && pivot_col < cols {
        //Find the pivot in the given column (row index with the highest value)
        let mut max_row = pivot_row;
        let mut max_val = 0;
        for i in pivot_row+1..rows-1 {
            let val = matrix[i][pivot_col].abs();
            if(val > max_val)
            {
                max_row = i;
                max_val = val;
            }
        }

        //Swap rows to bring the max element to the pivot position
        if(max_row != pivot_row)
        {
            let temp1 = matrix[pivot_row].clone();
            let temp2 = matrix[max_row].clone();
            matrix[max_row] = temp1;
            matrix[pivot_row] = temp2;
        }

        //Check if the pivot is zero
        if(max_val == 0)
        {
            //Matrix is singular or all zeros in column, move to next column
            pivot_col += 1;
            continue
        }

        //Normalize pivot row (make pivot value 1)
        let pivot_value = matrix[pivot_row][pivot_col];
        for cur_column in pivot_col..cols-1 {
            matrix[pivot_row][cur_column] = matrix[pivot_row][cur_column] / pivot_value;
        }

        //Eliminate other entries in the pivot column
        for cur_row in 0..rows-1 {
            if cur_row != pivot_row {
                let factor = matrix[cur_row][pivot_col];
                for cur_column in pivot_col..cols-1 {
                    matrix[cur_row][cur_column] = matrix[cur_row][cur_column] - factor * matrix[pivot_row][pivot_col];
                }
            }
        }

        //Increment pivot positions
        pivot_row += 1;
        pivot_col += 1;
    }

    return matrix;
}

fn get_solutions(
    matrix: &Vec<Vec<i16>>, // In row-echelon form
    height: usize,
    width: usize,
) -> Vec<Option<i32>>
{

    let num_vars = width - 1;
    let mut solutions: Vec<Option<i32>> = vec![None; num_vars];

    // for row_idx in 0..height {
    //     let row = &matrix[row_idx];
    //     let constant = row[num_vars];
    //
    //     let coefficients = row.iter()
    //       .enumerate()
    //       .filter(|(index, coefficient)| return **coefficient != 0 && *index < num_vars)
    //       .collect::<Vec<_>>();
    //
    //     if(coefficients.len() == 1)
    //     {
    //         let (index, coefficient) = coefficients[0];
    //         solutions[index] = Some((constant / *coefficient) as i32);
    //     }
    // }

    loop {
        let mut found_more_solutions = false;

        for row_idx in 0..height {
            let row = &matrix[row_idx];

            let equations = row.iter()
              .enumerate()
              .filter(|(index, coefficient)| return **coefficient != 0 && *index < num_vars)
              .map(|(index, coefficient)| {
                  return (index, coefficient, solutions[index])
              })
              .collect::<Vec<_>>();

            let missing_terms = equations.iter()
              .filter(|(_index, _coefficient, solution)| solution.is_none())
              .collect::<Vec<_>>();

            //If the number of missing terms is exactly 1, we can solve for it
            if(missing_terms.iter().count() == 1)
            {
                let constant = row[num_vars];
                let mut result = constant;

                let combined_terms = equations.iter()
                  .filter(|(_index, _coefficient, solution)| solution.is_some())
                  .map(|(index, coefficient, solution)| solution.unwrap() * **coefficient as i32)
                  .sum::<i32>();

                let missing_index = **missing_terms.iter()
                  .map(|(index, _coefficient, _solution)| index)
                  .collect::<Vec<_>>()
                  .first()
                  .unwrap();
                let missing_coefficient = row[missing_index];

                result -= combined_terms as i16;
                result *= missing_coefficient;
                solutions[missing_index as usize] = Some(result as i32);
                found_more_solutions = true;
            }
        }

        if !found_more_solutions {
            break;
        }
    }

    return solutions;
}

// fn get_free_variables(
//     matrix: &Vec<Vec<i16>>, // In row-echelon form
//     height: usize,
//     width: usize,
// )
// {
//     let mut pivot_cols: Vec<u16> = Vec::new();
//     let mut cur_row = 0;
//
//     for cur_column in 0..width-1 {
//         if(cur_row < height)
//         {
//             let value = matrix[cur_row][cur_column];
//             if(value == 1)
//             {
//                 pivot_cols.push(cur_row as u16);
//                 cur_row += 1;
//             }
//         }
//     }
//
//     let mut free_variables = Vec::new();
//     for cur_column in 0..height-1 {
//         if !pivot_cols.contains(&(cur_column as u16)) {
//             free_variables.push(cur_column as u16);
//         }
//     }
//
//     return free_variables;
// }


fn get_neighbours_2(
    machine: &Machine,
    current: &Point,
    goal: &Point,
    solutions: &Vec<Option<i32>>
) -> Vec<(u16, Point)>
{
    let mut results = Vec::new();

    for (index, solution) in solutions.iter().enumerate() {
        if(solution.is_none())
        {
            let button = &machine.buttons[index];
            let mut neighbour = current.clone();
            let mut neighbour_valid = true;
            let mut num_presses = 0;

            for wiring in button.get_button_wirings() {
                neighbour[wiring as usize] += 1;

                if neighbour[wiring as usize] > goal[wiring as usize]
                {
                    neighbour_valid = false;
                    break;
                }
            }

            if neighbour_valid {
                results.push((1, neighbour))
            }
        }
    }

    // for button in &machine.buttons {
    //     let mut num_presses = 1;
    //
    //     loop {
    //         let mut neighbour_pt: Point = current.clone();
    //         let mut neighbour_valid = true;
    //         let mut g_cost = 0;
    //
    //         for wiring in button.get_button_wirings() {
    //             neighbour_pt[wiring as usize] += num_presses;
    //             g_cost += num_presses;
    //
    //             if neighbour_pt[wiring as usize] > goal[wiring as usize]
    //             {
    //                 neighbour_valid = false;
    //                 break;
    //             }
    //         }
    //
    //         if !neighbour_valid {
    //             break;
    //         }
    //
    //         results.push((g_cost, neighbour_pt));
    //         num_presses = num_presses + 1;
    //     }
    // }

    return results;
}

fn astar_2(
    machine: &Machine,
    start: &Point,
    goal: &Point,
    solutions: &Vec<Option<i32>>
) -> u32
{
    let mut open_set = PriorityQueue::new();
    let mut closed_set: HashSet<Point> = HashSet::new();
    let mut g_scores: HashMap<Point, u32> = HashMap::new();
    let mut came_from: HashMap<Point, Point> = HashMap::new();

    let initial_h = heuristic(start, goal);
    open_set.push(start.clone(), Reverse(initial_h));
    g_scores.insert(start.clone(), 0);

    while !open_set.is_empty() {
        let current = open_set.pop().unwrap().0;

        // println!("{:?}", open_set.clone().into_iter().collect::<Vec<_>>());
        if current == *goal {
            //TODO - Reconstruct path? Don't actually need to
            let g_score = g_scores[&current];

            let mut c = Some(goal);
            while c.is_some() {
                println!("came from {:?} {}", c.unwrap(), g_scores[c.unwrap()]);
                c = came_from.get(c.unwrap())
            }

            return g_score;
        }

        closed_set.insert(current.clone());

        let neighbours = get_neighbours_2(machine, &current, &goal, &solutions);

        //Find neighbours of the current state
        for neighbour in &neighbours {
            let g_cost = neighbour.0;
            let neighbour_pt = &neighbour.1;

            //If neighbour is not a valid neighbour or is in the closed set, we skip it
            if(closed_set.contains(neighbour_pt))
            {
                // println!("SKIP {}, {}", !neighbour_valid, closed_set.contains(&neighbour_pt));
                continue;
            }

            //TODO - Could improve this by generating neighbours as n presses of button?
            //G score is previous score + 1 as each neighbour requires 1 iteration
            let tentative_g = g_scores.get(&current).unwrap() + g_cost as u32;
            let h_cost = heuristic(&neighbour_pt, goal);
            let f_cost = tentative_g + h_cost;

            let open_set_neighbour = open_set.get(neighbour_pt);
            //If we have not yet explored this neighbour, add it to the open set
            if open_set_neighbour.is_none() {
                open_set.push(neighbour_pt.clone(), Reverse(f_cost));
            }
            // This is worse than current method to this neighbour, ignore it
            else if(tentative_g >= g_scores[open_set_neighbour.unwrap().0]) {
                continue
            }

            // We found a better solution, override current solution
            came_from.insert(neighbour_pt.clone(), current.clone());
            let old_opt = open_set.change_priority(neighbour_pt, Reverse(f_cost));
            if old_opt.is_none()
            {
                println!("FUCKIN PROBLEM HOMIE");
            }
            g_scores.insert(neighbour_pt.clone(), tentative_g);
        }

    }

    return u32::MAX;
}

fn task2_attempt3(file_input: &String) -> i64 {
    let machines = parse_machines(file_input, true);
    let mut total_result = 0;

    for (index, machine) in machines.iter().enumerate() {
        let mut matrix: Vec<Vec<i16>> = vec![vec![0; machine.buttons.len() + 1]; machine.joltages.len()];

        for (joltage_index, joltage) in machine.joltages.iter().enumerate() {
            let row_len = matrix[joltage_index].len();
            matrix[joltage_index][row_len - 1] = *joltage as i16;
        }

        for (button_idx, button) in machine.buttons.iter().enumerate() {
            for wiring in button.get_button_wirings() {
                matrix[wiring as usize][button_idx] = 1;
            }
        }

        let m = matrix.len();
        let n = matrix[0].len();
        let elim_matrix = gaussian_elimination(matrix, m, n);
        println!("Gaussian {:?} ", elim_matrix);
        let solutions = get_solutions(&elim_matrix, m, n);
        println!("Solutions {:?} ", solutions);

        let mut start: Point = vec![0; machine.joltages.len()];
        let mut num_initial_presses = 0;
        for (index, num_presses) in solutions.iter().enumerate() {
            if(num_presses.is_some())
            {
                let button = &machine.buttons[index];
                for wiring in button.get_button_wirings() {
                    start[wiring as usize] += num_presses.unwrap() as u16;
                }
                num_initial_presses += num_presses.unwrap() as u16;
            }
        }

        println!("Start {:?} ", start);

        let mut result: u64 = num_initial_presses as u64;
        if(start != machine.joltages)
        {
            result += astar_2(&machine, &start, &machine.joltages, &solutions) as u64;
        }


        //
        // let start = vec![0; machine.joltages.len()];
        // let result = astar(&machine, &start, &machine.joltages);

        println!("Result {:?} ", result);

        println!("Complete {}/{}", index+1, machines.len());

        total_result += result;
    }

    return total_result as i64;
}

pub fn task2(file_input: &String) -> i64 {

    let machines = parse_machines(file_input, true);
    let mut total: i64 = 0;

    for machine in &machines {
        let mut solver_variables = variables!();

        let button_presses_variables: Vec<Variable> = machine.buttons.iter()
          .map(|_| solver_variables.add(variable().min(0).integer()))
          .collect();

        println!("Solver variables {:?}", button_presses_variables);

        //Express that calculates the total number of button presses based on the sum of how
        //many times each button was pressed
        let total_presses: Expression = button_presses_variables.iter().sum();

        let mut problem = solver_variables
          .minimise(total_presses)
          .using(default_solver); // Uses default iBM cbc solver

        for (joltage_idx, joltage) in machine.joltages.iter().enumerate() {
            //Defines an expression that adds up the button pressess of each button connected
            //to this joltage output to calculate the final total
            let mut expression = Expression::from(0);

            for (button_idx, button) in machine.buttons.iter().enumerate() {
                for wiring in button.get_button_wirings() {
                    if(wiring as usize == joltage_idx)
                    {
                        expression = expression.add(button_presses_variables[button_idx]);
                    }
                }
            }

            //The number of button presses linked to this joltage input has to sum
            //to the expected joltage output
            problem.add_constraint(expression.eq(*joltage as i32));
        }

        let solution = problem.solve().unwrap();

        let result = button_presses_variables.iter()
          .map(|variable| {
              return solution.value(*variable) as i32
          })
          .sum::<i32>();

        total += result as i64;
    }

    return total;



    // task2_attempt1(file_input);

    //Generate all button press sequences,
    //Store how many button presses it took to get to each sequence (minimum)
    //For every sequence. Minus the current sequence from the goal sequence,
    //If the result is all zeros OR a cached sequence, we can calculate the number
    //of presses

    //Select numbers in the sequence (starting with smallest is probably easiest)
    //Find the possible combinations of buttons that would get that number to zero
    //without putting any other number in the negative. One of those sub-solutions
    //must be part of the whole solution.
    //You could add a constraint for say "button 1 & button 2 need to be pressed 4 times in total"

    //Pressing buttons with more wires is always more valuable at reducing the press
    //count than those with less wires, prioritize searching buttons with more wires?

    //If all the numbers have a common factor, then however many button presses it
    //takes to solve to the soltuion where each number is divided by that factor, then
    //multiply the button presses by that factor will be correct
    //i.e. if it takes 10 presses to get to {1,2,4,2,1}
    // =>  then it takes 20 presses to get to {2,4,8,4,2}

    //Dynamic programming approach:
    // What is the closest I can get to the solution in 1 button press without going over?
    // What is the closest I can get to the solution in 2 button press without going over?
    // Can I use those previous results to solve the final result?
    // I don't think so as 5 button presses might have a fundamentally different sequence
    // than 4? Maybe there's a different way to formulate the problem?



    //Given this input:
    //[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
    //Arrange inputs into a simultaneous equation, we know each button
    //is a value between 0 & the greatest equals of an equation it is
    //a part of
    //i.e. b1 + b3 + b4 = 7
    //i.e. b3 + b4 = 5
    //i.e. b1 + b2 + b4 + b5 = 12
    //i.e. b1 + b2 + b5 = 7
    //i.e. b1 + b3 + b5 = 2

    //Might be able to do this as a matrix?
    //|1 0 1 1 0|   |b1|   |7 |
    //|0 0 1 1 0|   |b2|   |5 |
    //|1 1 0 1 1| x |b3| = |12|
    //|1 1 0 0 1|   |b4|   |7 |
    //|1 0 1 0 1|   |b5|   |2 |

    //b1 = max of 2
    //b2 = max of 7
    //b3 = max of 2
    //b4 = max of 4
    //b5 = max of 2
    //720 iterations to solve

    //There must be a minimum of 12 iterations given the input. So we can exclude some combinations?
    //Not sure exactly how to do that?
    //Can conclude the final result is between 12 & 17 iterations based on all my maximums

    //Map button to number of max iterations remaining, i.e
    // b1_max => (7+2+4+2) => 15
    // b2_max => (2+4+2) => 8
    // b3_max => (4+2) => 6
    // b4_max => (2) => 2
    // b5_max => 0
    //therefore we can do the iterations as \

    //Example
    // max_b1 = 8  (5) (minimum should be 7, num_iterations_left - max_buttons_left)
    // max_b2 = 5  (0) (assuming b1 was 8, then min number of iterations is 4
    // max_num_iterations = 12

    // while(true)
    //   for each i in 0..max_b1
    //    num_iterations -= i
    //    for each j in 0..max(max_b2, num_iterations)
    //        DO CHECK
    //    num_iterations += i
    // (Did not find a solution, increment the number of iterations)
    // num_iterations += 1
    //
    //Astar
    // Heuristic:
    //  max of (joltage(i)) - (expected(joltage(i))

    //Idea what if we pre-calculate 0 cost diffs?



    return 1;
}