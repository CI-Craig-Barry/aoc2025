use std::fmt::Debug;
use std::fmt;
use aoc2025::utils;

pub fn get_details() -> utils::ExecDetails {
  return utils::ExecDetails {
    day: 8,
    sample: include_str!("sample").to_string(),
    sample2: None,
    input: include_str!("input").to_string(),
    task1_function: task1,
    task2_function: task2,
    task1_sample_expected: 40,
    task1_input_expected: 69192,
    task2_sample_expected: 25272,
    task2_input_expected: 7264308110,
  };
}

#[derive(Debug)]
struct Point {
  id: i32,
  x: i32,
  y: i32,
  z: i32
}

impl Point {
  // fn manhattan_dist(&self, other: &Point) -> i32 {
  //   return (self.x - other.x).abs() +
  //     (self.y - other.y).abs() +
  //     (self.z - other.z).abs();
  // }

  fn euclidean_distance(&self, other: &Point) -> f64 {
    return (((self.x as i64 - other.x as i64).pow(2) +
      (self.y as i64 - other.y as i64).pow(2) +
      (self.z as i64 - other.z as i64).pow(2)) as f64).sqrt();
  }
}

impl fmt::Display for Point {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({}, {}, {})", self.x, self.y, self.z)
  }
}

#[derive(Debug)]
struct PointDistance {
  pt1: i32,
  pt2: i32,
  dist: i32
}

fn main() {
  utils::exec(&get_details());
}

fn sort_circuit_sizes(connections: &Vec<Vec<i32>>) -> Vec<i32>
{
  let mut checked: Vec<bool> = vec![false; connections.len()];
  let mut sizes: Vec<i32> = Vec::new();

  for (index, _connected_ids) in connections.iter().enumerate() {
    if !checked[index as usize] {
      // println!("--Start circuit--");
      sizes.push(find_circuit_recur(0, index as i32, &connections, &mut checked));
      // println!("\n--End circuit--");
    }
  }

  return sizes;
}

fn find_circuit_recur(
  mut cur_size: i32,
  cur_idx: i32,
  connections : &Vec<Vec<i32>>,
  checked : &mut Vec<bool>,
) -> i32
{
  // print!("{}, ", cur_idx);
  cur_size += 1;
  checked[cur_idx as usize] = true;

  let connected_ids : &Vec<i32> = &connections[cur_idx as usize];
  for downstream_id in connected_ids {
    if !checked[*downstream_id as usize]
    {
      cur_size = find_circuit_recur(cur_size, *downstream_id, connections, checked);
    }
  }

  return cur_size;
}


pub fn task1(file_input: &String) -> i64 {
  let mut points : Vec<Point> = Vec::new();

  for (index, line) in file_input.as_str().lines().enumerate() {
    let mut split = line.split(",");
    let point: Point = Point {
      id: index as i32,
      x: split.next().unwrap().parse::<i32>().unwrap(),
      y: split.next().unwrap().parse::<i32>().unwrap(),
      z: split.next().unwrap().parse::<i32>().unwrap()
    };

    points.push(point);
  }

  let num_connections = if points.len() > 100 {1000} else {10};

  let mut pt_distances: Vec<PointDistance> = Vec::new();

  for i in 0..points.len() {
    for j in i+1..points.len() {
      let pt1 : &Point = &points[i];
      let pt2 : &Point = &points[j];
      let dist = pt1.euclidean_distance(&pt2);
      pt_distances.push(PointDistance { pt1: pt1.id, pt2: pt2.id, dist: dist as i32 });
    }
  }

  pt_distances.sort_by(|dist_a, dist_b| dist_a.dist.cmp(&dist_b.dist));
  // println!("point distances: {}", pt_distances.len());
  // println!("{:?}", pt_distances);

  let mut connections : Vec<Vec<i32>> = Vec::new();
  for _i in 0..points.len() {
    connections.push(Vec::new());
  }

  for i in 0..num_connections {
    let pt_dist = &pt_distances[i];
    connections[pt_dist.pt1 as usize].push(pt_dist.pt2);
    connections[pt_dist.pt2 as usize].push(pt_dist.pt1);

    // println!("connection: {} -> {}", points[pt_dist.pt1 as usize], points[pt_dist.pt2 as usize]);
  }
  // println!("{:?}", connections);

  let mut sizes = sort_circuit_sizes(&connections);
  sizes.sort_by(|a, b| b.cmp(a));
  // println!("sizes: {:?}", sizes);

  let mut result = 1;
  for i in 0..3 {
    result *= sizes[i];
  }

  return result as i64;
}

pub fn task2(file_input: &String) -> i64 {
  let mut points : Vec<Point> = Vec::new();

  for (index, line) in file_input.as_str().lines().enumerate() {
    let mut split = line.split(",");
    let point: Point = Point {
      id: index as i32,
      x: split.next().unwrap().parse::<i32>().unwrap(),
      y: split.next().unwrap().parse::<i32>().unwrap(),
      z: split.next().unwrap().parse::<i32>().unwrap()
    };

    points.push(point);
  }

  let mut pt_distances: Vec<PointDistance> = Vec::new();

  for i in 0..points.len() {
    for j in i+1..points.len() {
      let pt1 : &Point = &points[i];
      let pt2 : &Point = &points[j];
      let dist = pt1.euclidean_distance(&pt2);
      pt_distances.push(PointDistance { pt1: pt1.id, pt2: pt2.id, dist: dist as i32 });
    }
  }

  pt_distances.sort_by(|dist_a, dist_b| dist_a.dist.cmp(&dist_b.dist));
  // println!("point distances: {}", pt_distances.len());
  // println!("{:?}", pt_distances);

  let mut connections : Vec<Vec<i32>> = Vec::new();
  for _i in 0..points.len() {
    connections.push(Vec::new());
  }

  let mut i = 0;
  loop {
    let pt_dist = &pt_distances[i];
    i += 1;
    connections[pt_dist.pt1 as usize].push(pt_dist.pt2);
    connections[pt_dist.pt2 as usize].push(pt_dist.pt1);

    let sizes = sort_circuit_sizes(&connections);
    if sizes.len() == 1 {
      return points[pt_dist.pt1 as usize].x as i64 *
        points[pt_dist.pt2 as usize].x as i64;
    }
  }
}