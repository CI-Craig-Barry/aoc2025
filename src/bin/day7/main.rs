use aoc2025::utils;

pub fn get_details() -> utils::ExecDetails {
  return utils::ExecDetails {
    day: 7,
    sample: include_str!("sample").to_string(),
    input: include_str!("input").to_string(),
    task1_function: task1,
    task2_function: task2,
    task1_sample_expected: 21,
    task1_input_expected: 1537,
    task2_sample_expected: 40,
    task2_input_expected: 18818811755665,
  };
}

fn main() {
  utils::exec(&get_details());
}


pub fn task1(file_input: &String) -> i64 {
  let splitter_locations = file_input.as_str().lines()
    .enumerate()
    .filter(|(index, _line)| return *index % 2 == 0 && *index > 0)
    .map(|(_, line)| {
      return line.chars()
        .map(|c| if c == '^' { true } else { false })
        .collect::<Vec<bool>>()
    })
    .collect::<Vec<Vec<bool>>>();

  let width = splitter_locations.get(0).unwrap().len();
  //Hack to calculate the middle in one line
  let start_idx = (width / 2) as usize;

  let mut last_beam_locations: Vec<bool>;
  let mut new_beam_locations: Vec<bool> = vec![false; width];
  new_beam_locations[start_idx] = true;

  let mut splits = 0;
  for row in splitter_locations {
    last_beam_locations = new_beam_locations.clone();
    new_beam_locations = vec![false; width];

    last_beam_locations
      .iter()
      .enumerate()
      .filter(|(_index, is_beam_location)| **is_beam_location)
      .for_each(|(index, _is_beam_location)| {
        if row[index]
        {
          splits += 1;
          new_beam_locations[index-1] = true;
          new_beam_locations[index+1] = true;
        }
        else
        {
          new_beam_locations[index] = true;
        }
      });
  }

  return splits;
}

pub fn task2(file_input: &String) -> i64 {
  let splitter_locations = file_input.as_str().lines()
    .enumerate()
    .filter(|(index, _line)| return *index % 2 == 0 && *index > 0)
    .map(|(_, line)| {
      return line.chars()
        .map(|c| if c == '^' { true } else { false })
        .collect::<Vec<bool>>()
    })
    .collect::<Vec<Vec<bool>>>();

  let width = splitter_locations.get(0).unwrap().len();
  //Hack to calculate the middle in one line
  let start_idx = (width / 2) as usize;

  let mut last_beam_locations: Vec<u64>;
  let mut new_beam_locations: Vec<u64> = vec![0; width];
  new_beam_locations[start_idx] = 1;

  for row in splitter_locations {
    last_beam_locations = new_beam_locations.clone();
    new_beam_locations = vec![0; width];

    for (location_index, num_beams) in last_beam_locations.iter().enumerate() {
      if *num_beams > 0
      {
        if *row.get(location_index as usize).unwrap() {
          // println!("{} {}", location_index, num_beams);
          new_beam_locations[location_index - 1] += num_beams;
          new_beam_locations[location_index + 1] += num_beams;
        }
        else {
          new_beam_locations[location_index] += num_beams;
        }
      }
    }
  }

  return new_beam_locations.iter().sum::<u64>() as i64;
}