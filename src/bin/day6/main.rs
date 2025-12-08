use std::cmp::max;
use aoc2025::utils;

pub fn get_details() -> utils::ExecDetails {
  return utils::ExecDetails {
    day: 6,
    sample: include_str!("sample").to_string(),
    input: include_str!("input").to_string(),
    task1_function: task1,
    task2_function: task2,
    task1_sample_expected: 4277556,
    task1_input_expected: 7229350537438,
    task2_sample_expected: 3263827,
    task2_input_expected: 11479269003550,
  };
}

fn main() {
  utils::exec(&get_details());
}

const PLUS: u8 = 1;
const MULTIPLY: u8 = 2;

pub fn decode_input(input: &String) -> (Vec<u32>, Vec<u8>)
{
  let mut inputs : Vec<u32> = Vec::new();
  let mut operations : Vec<u8> = Vec::new();

  for line in input.as_str().lines() {
    let first_char = line.chars().next().unwrap();
    if first_char == '*' || first_char == '+'
    {
      for cur_char in line.chars() {
        if cur_char == '*' {
          operations.push(MULTIPLY);
        }
        else if cur_char == '+' {
          operations.push(PLUS);
        }
      }
    }
    else
    {
      let mut string_buffer: String = String::new();
      for cur_char in line.chars() {
        if !cur_char.is_whitespace()
        {
          string_buffer.push(cur_char);
        }
        else if !string_buffer.is_empty()
        {
          inputs.push((&string_buffer).parse::<u32>().unwrap());
          string_buffer.clear();
        }
      }
      inputs.push((&string_buffer).parse::<u32>().unwrap());
    }
  }

  return (inputs, operations);
}

pub fn task1(file_input: &String) -> i64 {
  let (inputs, operations) = decode_input(file_input);

  let width = operations.len();
  let height = inputs.len() / width;
  let mut total: u64 = 0;

  //Iterate rows
  for x in 0..width {
    let operation = operations[x];
    let mut col_result: u64 = if operation == PLUS {0} else {1};

    //Iterate columns
    for y in 0..height {
      let input_index = (y * width) + x;
      let input: u64 = inputs[input_index] as u64;

      if operation == PLUS {
        col_result += input;
      }
      else {
        col_result *= input;
      }
    }

    total += col_result;
  }

  return total as i64;
}

pub fn task2(input: &String) -> i64 {
  //Super lazy
  const WIDTH : usize = 4096;
  const HEIGHT: usize = 8;
  let mut matrix : [[i8; HEIGHT]; WIDTH] = [[-1; HEIGHT]; WIDTH];
  let mut operations : [char; WIDTH] = [' '; WIDTH];

  let mut height = 0;
  let mut width = 0;
  input.as_str().lines()
    .enumerate()
    .for_each(|(line_idx, line)| {
      height = max(height, line_idx);
      line.as_bytes().iter().enumerate().for_each(|(byte_idx, byte)| {
        if *byte >= b'0' && *byte <= b'9'
        {
          width = max(width, byte_idx + 1);
          matrix[byte_idx][line_idx] = (*byte - b'0') as i8;
        }
        else if *byte != b' '
        {
          operations[byte_idx] = *byte as char;
        }
      })
    });


  let mut total: i64 = 0;
  let mut cur_total: i64 = 0;
  let mut last_operation = ' ';
  //Iterate columns (add 1 to add an empty column to the end)
  for x in 0..(width+1) {
    let mut operation = operations[x];
    if operation == ' '
    {
      operation = last_operation
    }
    else
    {
      last_operation = operation;
      if operation == '*'
      {
        cur_total = 1;
      }
      else
      {
        cur_total = 0;
      }
    }

    let mut has_value: bool = false;
    let mut cur_val: i64 = 0;

    //Iterate rows
    for y in 0..height {
      let data = matrix[x][y] as i64;

      if data != -1
      {
        has_value = true;
        cur_val *= 10;
        cur_val += data;
      }
    }

    if has_value
    {
      if operation == '*'
      {
        cur_total *= cur_val;
      }
      else
      {
        cur_total += cur_val;
      }
    }
    else
    {
      total += cur_total;
    }
  }

  return total;
}