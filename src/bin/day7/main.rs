use aoc2025::utils;

pub fn get_details() -> utils::ExecDetails {
  return utils::ExecDetails {
    day: 8,
    sample: include_str!("sample").to_string(),
    input: include_str!("input").to_string(),
    task1_function: task1,
    task2_function: task2,
    task1_sample_expected: 0,
    task1_input_expected: 0,
    task2_sample_expected: 0,
    task2_input_expected: 0,
  };
}

fn main() {
  utils::exec(&get_details());
}


pub fn task1(file_input: &String) -> i64 {
  return 1;
}

pub fn task2(file_input: &String) -> i64 {
  return 1;
}