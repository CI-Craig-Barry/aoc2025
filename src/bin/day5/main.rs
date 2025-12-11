use std::cmp::{max};
use aoc2025::utils;

pub fn get_details() -> utils::ExecDetails {
  return utils::ExecDetails {
    day: 5,
    sample: include_str!("sample").to_string(),
    sample2: None,
    input: include_str!("input").to_string(),
    task1_function: task1,
    task2_function: task2,
    task1_sample_expected: 3,
    task1_input_expected: 643,
    task2_sample_expected: 14,
    task2_input_expected: 342018167474526,
  };
}

fn main() {
  utils::exec(&get_details());
}

pub fn task1(input: &String) -> i64 {
  let mut ranges: Vec<(u64, u64)> = Vec::new();
  let mut available_ingredient_ids:Vec<u64> = Vec::new();
  let mut all_ranges_found: bool = false;

  for line in input.lines() {
    if !all_ranges_found
    {
      if line.is_empty()
      {
        all_ranges_found = true;
        continue;
      }

      let dash_idx = line.find("-").unwrap();
      //unwrap will panic if parsing failed
      let min_range = line[0..dash_idx].parse::<u64>().unwrap();
      let max_range = line[dash_idx+1..].parse::<u64>().unwrap();

      ranges.push((min_range, max_range));
    }
    else
    {
      let available_ingredient_id: u64 = line.parse().unwrap();
      available_ingredient_ids.push(available_ingredient_id);
    }
  }

  let mut num_fresh = 0;

  for ingredient_id in available_ingredient_ids {
    for (min_range, max_range) in &ranges {
      if ingredient_id >= *min_range && ingredient_id <= *max_range
      {
        num_fresh += 1;
        break;
      }
    }
  }

  return num_fresh;
}

pub fn task2(input: &String) -> i64 {
  let mut ranges: Vec<(u64, u64)> = Vec::with_capacity(200);

  for line in input.lines() {
    let opt_dash_idx = line.find("-");
    if opt_dash_idx == None
    {
      break;
    }
    let dash_idx = opt_dash_idx.unwrap();

    //unwrap will panic if parsing failed
    let min_range = line[0..dash_idx].parse::<u64>().unwrap();
    let max_range = line[dash_idx+1..].parse::<u64>().unwrap() + 1;

    ranges.push((min_range, max_range));
  }

  //Assort minimum ranges in ascending order
  ranges.sort_by(|a, b| a.0.cmp(&b.0));

  let mut num_ids = 0;
  let mut max_max_id = 0;
  let mut min_id;
  let mut max_id;
  let mut ids_in_range;

  for (min_range, max_range) in ranges {
    min_id = max(min_range, max_max_id);
    max_id = max(max_range, max_max_id);

    ids_in_range = max(max_id - min_id, 0);
    num_ids += ids_in_range;

    max_max_id = max(max_max_id, max_range);
  }

  return num_ids as i64;
}