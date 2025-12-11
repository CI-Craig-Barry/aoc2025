use std::cmp::{max, min};
use aoc2025::utils;
use std::collections::{HashSet};

pub fn get_details() -> utils::ExecDetails {
    return utils::ExecDetails {
        day: 10,
        sample: include_str!("sample").to_string(),
        input: include_str!("input").to_string(),
        task1_function: task1,
        task2_function: task2,
        task1_sample_expected: 7,
        task1_input_expected: 461,
        task2_sample_expected: 33,
        task2_input_expected: 0,
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
pub fn task2(file_input: &String) -> i64 {
    task2_attempt1(file_input);

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



    return 1;
}