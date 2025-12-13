use std::fmt::{Debug, Formatter};
use aoc2025::utils;

pub fn get_details() -> utils::ExecDetails {
    return utils::ExecDetails {
        day: 12,
        sample: include_str!("sample").to_string(),
        sample2: None,
        input: include_str!("input").to_string(),
        task1_function: task1,
        task2_function: task2,
        task1_sample_expected: 2,
        task1_input_expected: 0,
        task2_sample_expected: 0,
        task2_input_expected: 0,
    };
}

const SHAPE_WIDTH: usize = 3;

#[derive(Debug)]
struct Shape {
    orientations: Vec<Grid>,
    edge_masks: Vec<Grid>
}

impl Shape {
    fn new(spaces: Vec<Vec<bool>>) -> Shape {
        let mut all_spaces: Vec<Vec<Vec<bool>>> = Vec::new();
        all_spaces.push(spaces.clone());

        for _orientation_idx in 0..3 {
            let last_spaces: &&Vec<Vec<bool>> = &all_spaces.last().unwrap();
            let mut new_spaces: Vec<Vec<bool>> = vec![vec![false; 3]; 3];

            //Lazy rotation code
            new_spaces[0][0] = last_spaces[2][0];
            new_spaces[1][0] = last_spaces[2][1];
            new_spaces[2][0] = last_spaces[2][2];

            new_spaces[0][1] = last_spaces[1][0];
            new_spaces[1][1] = last_spaces[1][1];
            new_spaces[2][1] = last_spaces[1][2];

            new_spaces[0][2] = last_spaces[0][0];
            new_spaces[1][2] = last_spaces[0][1];
            new_spaces[2][2] = last_spaces[0][2];

            all_spaces.push(new_spaces);
        }

        //If the shape is identical on the transpose, remove duplicate orientations
        if all_spaces[0] == all_spaces[2]
        {
            all_spaces.pop();
            all_spaces.pop();
        }

        //TODO - Flip orientations?

        let mut orientations: Vec<Grid> = Vec::new();

        //Convert Vec<Vec<bool>> representations to bitmask representations
        for spaces in all_spaces {
            let mut lines: Vec<u64> = Vec::new();
            for (_line_index, line_bools) in spaces.iter().enumerate() {
                let mut mask = 0u64;
                for b in line_bools.iter().rev() {
                    mask >>= 1;
                    mask |= if *b {1 << 63} else {0};
                }
                lines.push(mask);
            }

            orientations.push(Grid {
                width: SHAPE_WIDTH as u16,
                height: SHAPE_WIDTH as u16,
                lines: lines.clone()
            });
        }

        let mut edge_masks: Vec<Grid> = Vec::new();
        let edge_mask_width = 5;

        for grid in &orientations {
            let mut edge_mask = Grid::new(edge_mask_width, edge_mask_width);

            for x in 0..SHAPE_WIDTH {
                for y in 0..SHAPE_WIDTH {
                    let is_filled = grid.is_pos_filled((x as u16, y as u16));

                    if(is_filled)
                    {
                        //Fill surrounding bits in the mask
                        edge_mask.set_bit((x as u16 + 2, y as u16 + 1));
                        edge_mask.set_bit((x as u16 + 0, y as u16 + 1));
                        edge_mask.set_bit((x as u16 + 1, y as u16 + 0));
                        edge_mask.set_bit((x as u16 + 1, y as u16 + 2));
                    }
                }
            }
            //Remove bits that overlap with the
            edge_mask.apply_mask((1, 1), &grid, false);
            edge_masks.push(edge_mask);
        }

        return Shape {
            orientations,
            edge_masks,
        }
    }
}

#[derive(Debug)]
struct Region {
    width: u16,
    height: u16,
    requirements: Vec<u16>,
}

struct Problem {
    width: u16,
    height: u16,
    lines: Vec<u64>
}

impl Problem {
    fn new(region: &Region) -> Problem {
        return Problem {
            width: region.width,
            height: region.width,
            lines: vec![0; region.height as usize]
        }
    }

    fn clear(&mut self)  {
        self.lines = vec![0; self.height as usize];
    }

    fn is_pos_filled(&self, pos: (u16, u16)) -> bool {
        let x = pos.0;
        let y = pos.1;

        if x < self.width && y < self.height
        {
            let row = self.lines[y as usize];
            let mask = 1u64 << (63 - x) as u64;
            return mask & row != 0;
        }

        return false;
    }

    // fn can_shape_fit(&self, middle_pos: (u16, u16), shape: &ShapeOrientation) -> bool {
    //     let x = middle_pos.0;
    //     let y = middle_pos.1;
    //
    //     //Assuming 3x3 bounding box for simplicity of calculations
    //     if x > 0 &&
    //       x < self.width - 1 &&
    //       y > 0 &&
    //       y < self.height - 1
    //     {
    //         //Will atleast fit on the grid, ensure we can actually fit it in empty spaces now
    //         for row_idx in 0..shape.lines.len() {
    //             let row_in_problem = y - 1 + row_idx as u16;
    //             let row_mask = self.lines[row_in_problem as usize];
    //
    //             let mut shape_mask = shape.lines[row_idx] as u64;
    //             //Shift 61 left would be equivalent to x==1
    //             shape_mask <<= 62 - x;
    //
    //             //If masks have no overlap, shape can be placed
    //             if(row_mask & shape_mask) != 0 {
    //                 return false;
    //             }
    //         }
    //     }
    //
    //     return false;
    // }
    //
    // fn apply_shape(&mut self, middle_pos: (u16, u16), shape: &ShapeOrientation) {
    //     //There is no validation, can_fit_shape should be called first
    //     let x = middle_pos.0;
    //     let y = middle_pos.1;
    //
    //     for row_idx in 0..shape.lines.len() {
    //         let row_in_problem = y - 1 + row_idx as u16;
    //         let mut shape_mask = shape.lines[row_idx] as u64;
    //         //Shift 61 left would be equivalent to x==1
    //         shape_mask <<= 62 - x;
    //
    //         //If masks have no overlap, shape can be placed
    //         self.lines[row_in_problem as usize] |= shape_mask;
    //     }
    // }
}

fn main() {
    utils::exec(&get_details());
}

fn parse_inputs(input: &String) -> (Vec<Region>, Vec<Shape>)
{
    let mut shapes: Vec<Shape> = Vec::new();
    let mut input_sections = input.split("\n\n").collect::<Vec<&str>>();

    //Parse shapes
    let last_index = input_sections.len() - 1;
    for section in &input_sections[..last_index] {
        let mut shape_section = section.lines().collect::<Vec<&str>>();
        shape_section.remove(0);
        let mut shape_spaces: Vec<Vec<bool>> = Vec::new();

        for shape_line in shape_section {
            let mut shape_spaces_line = Vec::new();
            for char in shape_line.chars() {
                if char == '#' {
                    shape_spaces_line.push(true);
                } else if char == '.' {
                    shape_spaces_line.push(false);
                }
            }
            shape_spaces.push(shape_spaces_line);
        }

        shapes.push(Shape::new(shape_spaces));
    }

    //Parse regions
    let mut regions: Vec<Region> = Vec::new();
    let regions_section = input_sections[input_sections.len() - 1];
    for line in regions_section.lines() {
        let parts = line.split(": ").collect::<Vec<&str>>();
        let size_part = parts[0];
        let requirements_part = parts[1];

        let size_parts = size_part.split("x").collect::<Vec<&str>>();
        let width = size_parts[0].parse::<u16>().unwrap();
        let height = size_parts[1].parse::<u16>().unwrap();

        let requirements: Vec<u16> = requirements_part.split(" ")
          .map(|r| r.parse::<u16>().unwrap())
          .collect();

        regions.push(Region {
            width,
            height,
            requirements,
        });
    }

    return (regions, shapes);
}

struct Grid {
    width: u16,
    height: u16,
    lines: Vec<u64>
}

impl Grid {
    fn new(width: u16, height: u16) -> Grid {
        return Grid {
            width,
            height,
            lines: vec![0; height as usize]
        }
    }

    fn iter(&self) -> GridIter<'_> {
        return GridIter::new(&self);
    }

    fn clear(&mut self)  {
        self.lines = vec![0; self.height as usize];
    }

    fn is_pos_filled(&self, pos: (u16, u16)) -> bool {
        let x = pos.0;
        let y = pos.1;

        if x < self.width && y < self.height
        {
            let row = self.lines[y as usize];
            let mask = 1u64 << (63 - x) as u64;
            return mask & row != 0;
        }

        return false;
    }

    fn set_bit(&mut self, pos: (u16, u16)) {
        let x = pos.0;
        let y = pos.1;

        let mask = 1u64 << (63 - x) as u64;
        self.lines[y as usize] |= mask;
    }

    fn apply_mask(&mut self, top_left_pos: (i16, i16), mask: &Grid, positive_mask: bool) {
        //There is no validation, can_fit_shape should be called first
        let x = top_left_pos.0;
        let y = top_left_pos.1;
        let row_bitmask = std::u64::MAX << 64 - self.width;

        for row_idx in 0..mask.height as i16 {
            let row_in_problem = y + row_idx;
            //Exceeds the length of the problem
            if(row_in_problem < 0 || row_in_problem >= self.height as i16) {
                continue;
            }

            let mut mask_bitwise = mask.lines[row_idx as usize];
            if(x >= 0)
            {
                mask_bitwise >>= x;
            }
            else
            {
                mask_bitwise <<= x.abs() as usize;
            }
            //Sanitize bitmask to keep it in the desired length & not affect things
            //outside the rows
            mask_bitwise &= row_bitmask;

            //Apply mask
            if(positive_mask) {
                self.lines[row_in_problem as usize] |= mask_bitwise;
            } else {
                self.lines[row_in_problem as usize] &= !mask_bitwise;
            }
        }
    }

    fn can_shape_fit(&self, top_left_pos: (i16, i16), mask: &Grid) -> bool {
        let x = top_left_pos.0;
        let y = top_left_pos.1;

        if  x <= (self.width - mask.width) as i16 &&
          y <= (self.height - mask.height) as i16
        {
            return self.count_overlaps(top_left_pos, mask) == 0;
        }

        return false;
    }

    fn count_overlaps(&self, top_left_pos: (i16, i16), mask: &Grid) -> u32 {
        //There is no validation, can_fit_shape should be called first
        let x = top_left_pos.0;
        let y = top_left_pos.1;

        let mut total: u32 = 0;

        for row_idx in 0..mask.height {
            let row_in_problem = y + row_idx as i16;
            //Exceeds the length of the problem
            if(row_in_problem < 0 || row_in_problem >= self.height as i16) {
                continue;
            }

            let mut mask_bitwise = mask.lines[row_idx as usize];
            //Account for negative positions
            if(x >= 0)
            {
                mask_bitwise >>= x;
            }
            else
            {
                mask_bitwise <<= x.abs() as usize;
            }

            let mask_result = mask_bitwise & self.lines[row_in_problem as usize];
            total += mask_result.count_ones();
        }

        return total;
    }

    fn create_edge_mask(&self) -> Grid {
        let mut edge_mask = Grid::new(self.width, self.height);

        for line_iter in 0..self.height {
            let line = self.lines[line_iter as usize];
            let mut new_line = line;
            new_line |= (line << 1);
            new_line |= (line >> 1);

            if(line_iter > 0)
            {
                new_line |= self.lines[line_iter as usize - 1];
            }
            if (line_iter < self.height - 1)
            {
                new_line |= self.lines[line_iter as usize + 1];
            }

            edge_mask.lines[line_iter as usize] = new_line;
        }

        edge_mask.apply_mask((0, 0), self, false);
        return edge_mask;
    }

    fn make_border_grid(grid: &Grid) -> Grid {
        let mut border_grid: Grid = Grid::new(grid.width, grid.height);

        for x in 0..border_grid.width {
            border_grid.set_bit((x, 0));
            border_grid.set_bit((x, grid.height - 1));
        }

        for y in 0..border_grid.height {
            border_grid.set_bit((0, y));
            border_grid.set_bit((grid.width - 1, y));
        }

        return border_grid;
    }

    fn blend_up(&mut self) {
        for line_iter in 0..self.height-1 {
            let next = self.lines[line_iter as usize + 1];
            self.lines[line_iter as usize] |= next;
        }
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.is_pos_filled((x, y)) {
                    output.push('#');
                }
                else {
                    output.push('.');
                }
            }

            output.push('\n');
        }

        return f.write_str(&output);
    }
}

struct GridIter<'a>
{
    data: &'a Grid,

    line_idx: u16,
    // last_line_mask: u64,
    x_idx: u16,
}

impl GridIter<'_>
{
    fn new(grid: &Grid) -> GridIter {
        return GridIter {
            data: grid,
            line_idx: 0,
            x_idx: 0,
        }
    }
}

impl<'a> Iterator for GridIter<'a> {
    type Item = (i16, i16);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            //We have reached the end of the grid
            if(self.line_idx == self.data.height)
            {
                return None;
            }

            //Shift line mask so the x_idx is in the MSB
            let mut line_mask = self.data.lines[self.line_idx as usize];
            line_mask <<= self.x_idx;

            //No 1s left on this row
            if(line_mask == 0)
            {
                self.line_idx += 1;
                self.x_idx = 0;
                continue;
            }

            //Count the number of zeros until the next one, this will tell us
            //how many x indices we need to skip to find the next 1
            let leading_zeros = line_mask.leading_zeros() as i16;
            let result = (self.x_idx as i16 + leading_zeros, self.line_idx as i16);
            self.x_idx += leading_zeros as u16 + 1;

            return Some(result);
        }
    }
}

fn heuristic(
    grid: &Grid,
    grid_border_mask: &Grid,
    pos: &(i16, i16),
    shape_mask: &Grid,
    edge_mask: &Grid
) -> u32 {
    return grid.count_overlaps((pos.0-1, pos.1-1), edge_mask);
    // return grid_border_mask.count_overlaps(*pos, shape_mask) +
    //     grid.count_overlaps((pos.0-1, pos.1-1), edge_mask) / 2;
}

fn can_be_solved(region: &Region, shapes: &Vec<Shape>) -> bool
{
    let mut grid = Grid::new(region.width, region.height);
    let mut search_space = Grid::new(region.width, region.height);
    //Start search space at (0,0)
    search_space.set_bit((0, 0));
    let border_grid = Grid::make_border_grid(&grid);

    let mut requirements = region.requirements.clone();
    let total_requirements = requirements.iter().sum();

    //Iterate for the total number of times we need to place shapes
    for i in 0..total_requirements {
        let mut best_pos: Option<(i16, i16)> = None;
        let mut best_orientation: Option<&Grid> = None;
        let mut best_shape_idx = 0;
        let mut best_h: i32 = -1;

        println!("Requirements {:?}", requirements);
        println!("Search Space:\n{:?}", search_space);
        println!();

        //Iterate over each requirement
        for (shape_idx, num_required_shapes) in requirements.iter().enumerate() {
            //If we don't need any more of this shape, don't attempt to place it
            if(*num_required_shapes == 0)
            {
                continue;
            }

            let shape = &shapes[shape_idx];
            //Find the best position for this shape in the search space
            for cur_pos in search_space.iter() {
                for (orientation_idx, orientation) in shape.orientations.iter().enumerate() {
                    if(grid.can_shape_fit(cur_pos, &orientation)) {
                        let h = heuristic(
                            &grid,
                            &border_grid,
                            &cur_pos,
                            &orientation,
                            &shape.edge_masks[orientation_idx]
                        ) as i32;

                        if(h > best_h)
                        {
                            best_h = h;
                            best_pos = Some(cur_pos);
                            best_shape_idx = shape_idx;
                            best_orientation = Some(orientation);
                        }
                    }
                }
            }
        }

        if(best_pos == None)
        {
            println!("Failed\n{:?}", grid);
            return false;
        }
        else
        {
            grid.apply_mask(best_pos.unwrap(), best_orientation.unwrap(), true);
            requirements[best_shape_idx] -= 1;
            println!("Placement \n{:?}", grid);
        }

        //The search space is
        search_space = grid.create_edge_mask();
        search_space.blend_up();
        //Optimize, never need to search the last 2 line
        search_space.lines[search_space.height as usize - 2] = 0;
        search_space.lines[search_space.height as usize - 1] = 0;
    }

    println!("Passed\n {:?}", grid);

    return true;
}

fn can_be_solved_2(region: &Region, shapes: &Vec<Shape>) -> bool {
    let area = region.width * region.height;
    let total_requirements: u16 = region.requirements.iter().sum::<u16>();

    if(area >= 9 * total_requirements)
    {
        return true;
    }

    return false;
}

pub fn task1(input: &String) -> i64 {
    let (regions, shapes) = parse_inputs(input);

    // can_be_solved(&regions[0], &shapes);

    let mut total_solvable = 0;

    for region in &regions {
        total_solvable += if can_be_solved_2(region, &shapes) {1} else {0};
    }

    return total_solvable;

    // can_be_solved(&regions[0], &shapes);

    // println!("shapes:{:?}", shapes);
    // println!("regions:{:?}", regions);

    // let mut problem = Problem::new(&regions[0]);

    // let region = &regions[1];
    // let mut grid = Grid::new(region.width, region.height);
    // grid.apply_mask((0, 2), &shapes[0].orientations[0], true);
    // grid.apply_mask((2, 2), &shapes[0].orientations[2], true);
    //
    // println!("problem: \n{:?}", grid);
    // println!("negative: \n{:?}", grid.create_edge_mask());
    // let mut blended_negative = grid.create_edge_mask();
    // blended_negative.blend_up();
    // println!("blended negative: \n{:?}", blended_negative);
    // for (x, y) in grid.iter() {
    //     println!("x: {}, y: {}", x, y);
    // }


    //440 - too low

    return 1;
}

pub fn task2(input: &String) -> i64 {
    return 1;
}