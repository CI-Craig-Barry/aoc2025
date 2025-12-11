use aoc2025::utils;

pub fn get_details() -> utils::ExecDetails {
  return utils::ExecDetails {
    day: 4,
    sample: include_str!("sample").to_string(),
    input: include_str!("input").to_string(),
    task1_function: task1,
    task2_function: task2,
    task1_sample_expected: 13,
    task1_input_expected: 1578,
    task2_sample_expected: 43,
    task2_input_expected: 10132,
  };
}

fn main() {
  utils::exec(&get_details());
}

fn is_toilet_paper(
  toilet_paper_positions: &Vec<bool>,
  x_pos: i32,
  y_pos: i32,
  width: i32
) -> bool
{
  let index = (y_pos * width) + x_pos;
  if x_pos >= 0 && x_pos < width &&
    y_pos >= 0 && index < toilet_paper_positions.len() as i32
  {
    return toilet_paper_positions[index as usize];
  }

  return false;
}

pub fn task1(input: &String) -> i64 {
  let mut toilet_paper_positions: Vec<bool> = Vec::new();
  let mut width: i32 = 0;

  for char in input.as_str().as_bytes() {
    if *char == b'@' {
      toilet_paper_positions.push(true);
    }
    else if *char == b'.' {
      toilet_paper_positions.push(false);
    }
    else if width == 0 {
      width = toilet_paper_positions.len() as i32;
    }
  }

  let mut result = 0;
  for index in 0..toilet_paper_positions.len()
  {
    if !toilet_paper_positions[index]
    {
      continue;
    }

    let mut count = 0;
    let index_i32: i32 = index as i32;
    let x_pos = index_i32 % width;
    let y_pos = index_i32 / width;
    count += if is_toilet_paper(&toilet_paper_positions, x_pos - 1, y_pos, width)              {1} else {0};
    count += if is_toilet_paper(&toilet_paper_positions, x_pos + 1, y_pos, width)              {1} else {0};
    count += if is_toilet_paper(&toilet_paper_positions, x_pos - 1, y_pos - 1, width)    {1} else {0};
    count += if is_toilet_paper(&toilet_paper_positions, x_pos, y_pos - 1, width)              {1} else {0};
    count += if is_toilet_paper(&toilet_paper_positions, x_pos + 1, y_pos - 1, width)    {1} else {0};
    count += if is_toilet_paper(&toilet_paper_positions, x_pos - 1, y_pos + 1, width)    {1} else {0};
    count += if is_toilet_paper(&toilet_paper_positions, x_pos, y_pos + 1, width)              {1} else {0};
    count += if is_toilet_paper(&toilet_paper_positions, x_pos + 1, y_pos + 1, width)    {1} else {0};

    if count < 4
    {
      result += 1;
    }
  }

  return result;
}

pub fn task2(input: &String) -> i64 {
  let mut toilet_paper_positions: Vec<bool> = Vec::new();
  let mut width: i32 = 0;

  for char in input.as_str().as_bytes() {
    if *char == b'@' {
      toilet_paper_positions.push(true);
    }
    else if *char == b'.' {
      toilet_paper_positions.push(false);
    }
    else if width == 0 {
      width = toilet_paper_positions.len() as i32;
    }
  }

  let mut removed = true;
  let mut result = 0;
  while removed {
    removed = false;

    for index in 0..toilet_paper_positions.len()
    {
      if !toilet_paper_positions[index]
      {
        continue;
      }

      let mut count = 0;
      let index_i32: i32 = index as i32;
      let x_pos = index_i32 % width;
      let y_pos = index_i32 / width;
      count += if is_toilet_paper(&toilet_paper_positions, x_pos - 1, y_pos, width)              {1} else {0};
      count += if is_toilet_paper(&toilet_paper_positions, x_pos + 1, y_pos, width)              {1} else {0};
      count += if is_toilet_paper(&toilet_paper_positions, x_pos - 1, y_pos - 1, width)    {1} else {0};
      count += if is_toilet_paper(&toilet_paper_positions, x_pos, y_pos - 1, width)              {1} else {0};
      count += if is_toilet_paper(&toilet_paper_positions, x_pos + 1, y_pos - 1, width)    {1} else {0};
      count += if is_toilet_paper(&toilet_paper_positions, x_pos - 1, y_pos + 1, width)    {1} else {0};
      count += if is_toilet_paper(&toilet_paper_positions, x_pos, y_pos + 1, width)              {1} else {0};
      count += if is_toilet_paper(&toilet_paper_positions, x_pos + 1, y_pos + 1, width)    {1} else {0};

      if count < 4
      {
        removed = true;
        result += 1;
        toilet_paper_positions[index] = false;
      }
    }
  }


  return result;
}