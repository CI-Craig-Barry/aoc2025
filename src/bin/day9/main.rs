use aoc2025::utils;

pub fn get_details() -> utils::ExecDetails {
    return utils::ExecDetails {
        day: 9,
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

struct Point {
    x: u64,
    y: u64
}

impl Point {
    fn manhattan_distance(self: &Point, other: &Point) -> u64
    {
        return ((self.x as i64 - other.x as i64).abs() +
            (self.y as i64 - other.y as i64).abs()) as u64;
    }
}

pub fn task1(file_input: &String) -> i64 {
    let points: Vec<Point> = file_input.lines()
        .map(|line| {
            let dash_idx = line.find(",").unwrap();
            let x_pos = line[0..dash_idx].parse::<u64>().unwrap();
            let y_pos = line[dash_idx+1..].parse::<u64>().unwrap();

            return Point {
                x: x_pos,
                y: y_pos
            };
        })
        .collect::<Vec<Point>>();

    return 1
}

pub fn task2(file_input: &String) -> i64 {
    return 1;
}