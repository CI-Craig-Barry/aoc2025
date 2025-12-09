use std::cmp::{max, min};
use aoc2025::utils;

pub fn get_details() -> utils::ExecDetails {
    return utils::ExecDetails {
        day: 9,
        sample: include_str!("sample").to_string(),
        input: include_str!("input").to_string(),
        task1_function: task1,
        task2_function: task2,
        task1_sample_expected: 50,
        task1_input_expected: 4763932976,
        task2_sample_expected: 24,
        task2_input_expected: 1501292304,
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

    fn area(self: &Point, other: &Point) -> u64
    {
        return (((self.x as i64 - other.x as i64).abs() + 1) *
          ((self.y as i64 - other.y as i64).abs() + 1)) as u64;
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

    let mut highest_area: u64 = 0;
    for i in 0..points.len() {
        for j in i+1..points.len() {
            let pt_1: &Point = &points[i];
            let pt_2: &Point = &points[j];
            // let area = ((pt_1.x as i64 - pt_2.x as i64).abs() + 1) *
            //   ((pt_1.y as i64 - pt_2 as i64) + 1);
            let area = pt_1.area(&pt_2);

            highest_area = max(highest_area, area);
        }
    }

    return highest_area as i64;
}

struct Box {
    left: i32,
    right: i32,
    bottom: i32,
    top: i32,
    area: u64
}
impl Box {
    fn new(
        left: i32,
        right: i32,
        bottom: i32,
        top: i32
    ) -> Self
    {
        Box {
            left,
            right,
            bottom,
            top,
            area: (right - left + 1) as u64 *
              (top - bottom + 1) as u64
        }
    }
}

struct Line {
    // The position on the axis (i.e. if its vertical then its x coordinate, if horizontal then its y coordinate)
    off_axis_coord: i32,
    // The min/max positions (i.e. the start/end of the line on the opposite axis of the line)
    axis_min_coord: i32,
    axis_max_coord: i32,
    // Whether the line is vertical or horizontal
    line_type: i8 /*(VERTICAL/HORIZONTAL)*/
}

impl Box {
    fn collides_with(self: &Box, line: &Line) -> bool {
        let min_l = if line.line_type == VERTICAL {self.left} else {self.bottom};
        let max_l = if line.line_type == VERTICAL {self.right} else {self.top};

        //If the vertical line is within the horizontal bounds
        //(or the horizontal line is within the vertical bounds)
        if (line.off_axis_coord > min_l) && (line.off_axis_coord < max_l) {
            //line.min_coord >= max_p Line is above box
            //line.max_coord <= min_p line is below box

            //Check if the line is within the actual bounds
            let min_p = if line.line_type == VERTICAL {self.bottom} else {self.left};
            let max_p = if line.line_type == VERTICAL {self.top} else {self.right};

            return !(line.axis_min_coord >= max_p || line.axis_max_coord <= min_p);
        }

        return false;
    }
}

const VERTICAL: i8 = 0;
const HORIZONTAL: i8 = 1;

fn make_box(pt_1: &Point, pt_2: &Point) -> Box {
    return Box::new(
        min(pt_1.x, pt_2.x) as i32,
        max(pt_1.x, pt_2.x) as i32,
        min(pt_1.y, pt_2.y) as i32,
        max(pt_1.y, pt_2.y) as i32,
    );
}

fn make_line(pt_1: &Point, pt_2: &Point) -> Line {
    return if pt_1.x == pt_2.x {
        Line {
            off_axis_coord: pt_1.x as i32,
            axis_min_coord: min(pt_1.y, pt_2.y) as i32,
            axis_max_coord: max(pt_1.y, pt_2.y) as i32,
            line_type: VERTICAL
        }
    } else {
        Line {
            off_axis_coord: pt_1.y as i32,
            axis_min_coord: min(pt_1.x, pt_2.x) as i32,
            axis_max_coord: max(pt_1.x, pt_2.x) as i32,
            line_type: HORIZONTAL
        }
    }
}

fn make_lines(points: &Vec<Point>) -> (Vec<Line>, Vec<Line>) {
    let mut vertical_lines: Vec<Line> = Vec::new();
    let mut horizontal_lines: Vec<Line> = Vec::new();

    let num_points = points.len();
    for index in 1..(num_points+1)
    {
        let prev_index = index - 1;
        let pt_1 = &points[prev_index];
        let pt_2 = &points[index % num_points];

        let line = make_line(pt_1, pt_2);
        if line.line_type == VERTICAL {
            vertical_lines.push(line);
        }
        else {
            horizontal_lines.push(line);
        }
    }

    return (vertical_lines, horizontal_lines);
}

pub fn task2(file_input: &String) -> i64 {

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
    //
    // horizontal_lines.sort_by(|l1, l2| {
    //     return l1.axis_coord.cmp(l2.axis_coord);
    // });
    // vertical_lines.sort_by(|l1, l2| {
    //     return l1.axis_coord.cmp(l2.axis_coord);
    // });

    let mut boxes:Vec<Box> = Vec::new();
    for i in 0..points.len() {
        for j in i+1..points.len() {
            let pt_1: &Point = &points[i];
            let pt_2: &Point = &points[j];

            boxes.push(make_box(pt_1, pt_2));
        }
    }

    //Sort descending by area
    boxes.sort_by(|b1, b2| {
        b2.area.cmp(&b1.area)
    });

    let lines = make_lines(&points);
    let horizontal_lines = &lines.0;
    let vertical_lines = &lines.1;

    //Could optimize by sorting horizontal lines & vertical lines by their off-axis coordinate
    //and then only need to check lines which are within the axis bounds (i.e. binary search into
    // the list to find any coordinate within axis & then iterate above & below that pivot point
    // to only include lines that are within atleast 1 correct dimension)

    for b in boxes {
        let mut collision = false;
        for line in horizontal_lines {
            if b.collides_with(&line)
            {
                collision = true;
                break;
            }
        }

        if !collision {
            for line in vertical_lines {
                if b.collides_with(&line)
                {
                    collision = true;
                    break;
                }
            }
        }

        if !collision {
            return b.area as i64;
        }
    }

    return 0;
}