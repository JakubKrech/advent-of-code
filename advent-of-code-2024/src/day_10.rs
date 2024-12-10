use std::collections::HashSet;

use crate::utilities::get_input_lines;

// https://adventofcode.com/2024/day/10

const DAY_STRING : &str = "day_10";
const USE_TEST_DATA : bool = false;

fn traverse_dup(grid : &mut[&mut[i32]], cur_y : usize, cur_x : usize) -> Vec<(usize, usize)> {
    let mut result : Vec<(usize, usize)> = vec![];

    let current_height : u32 = grid[cur_y][cur_x].to_string().parse().expect("Failed to parse");

    if current_height == 9 {
        result.push((cur_y, cur_x));
        return result;
    }

    // Go up
    if grid[cur_y - 1][cur_x] != -1 {
        let up_val : u32 = grid[cur_y - 1][cur_x].to_string().parse().expect("Failed to parse");

        if up_val == current_height + 1 {
            result.extend(traverse_dup(grid, cur_y - 1, cur_x));
        }
    }

    // Go right
    if grid[cur_y][cur_x + 1] != -1 {
        let up_val : u32 = grid[cur_y][cur_x + 1].to_string().parse().expect("Failed to parse");

        if up_val == current_height + 1 {
            result.extend(traverse_dup(grid, cur_y, cur_x + 1));
        }
    }

    // Go down
    if grid[cur_y + 1][cur_x] != -1 {
        let up_val : u32 = grid[cur_y + 1][cur_x].to_string().parse().expect("Failed to parse");

        if up_val == current_height + 1 {
            result.extend(traverse_dup(grid, cur_y + 1, cur_x));
        }
    }

    // Go left
    if grid[cur_y][cur_x - 1] != -1 {
        let up_val : u32 = grid[cur_y][cur_x - 1].to_string().parse().expect("Failed to parse");

        if up_val == current_height + 1 {
            result.extend(traverse_dup(grid, cur_y, cur_x - 1));
        }
    }

    return result;
}

fn traverse(grid : &mut[&mut[i32]], cur_y : usize, cur_x : usize) -> HashSet<(usize, usize)> {
    let mut result : HashSet<(usize, usize)> = HashSet::new();

    let current_height : i32 = grid[cur_y][cur_x];

    // println!("Traverse: {},{}  Current Height: {}", cur_y, cur_x, current_height);

    if current_height == 9 {
        result.insert((cur_y, cur_x));
        // println!("Found: {},{}", cur_y, cur_x);
        return result;
    }
    
    // For each direction try to find level higher by 1

    // Go up
    if grid[cur_y - 1][cur_x] != -1 {
        if grid[cur_y - 1][cur_x] == current_height + 1 {
            result.extend(traverse(grid, cur_y - 1, cur_x));
        }
    }

    // Go right
    if grid[cur_y][cur_x + 1] != -1 {
        if grid[cur_y][cur_x + 1] == current_height + 1 {
            result.extend(traverse(grid, cur_y, cur_x + 1));
        }
    }

    // Go down
    if grid[cur_y + 1][cur_x] != -1 {
        if grid[cur_y + 1][cur_x] == current_height + 1 {
            result.extend(traverse(grid, cur_y + 1, cur_x));
        }
    }

    // Go left
    if grid[cur_y][cur_x - 1] != -1 {
        if grid[cur_y][cur_x - 1] == current_height + 1 {
            result.extend(traverse(grid, cur_y, cur_x - 1));
        }
    }

    return result;
}

#[allow(dead_code)]
pub fn part_1() -> String
{
    let input = get_input_lines(DAY_STRING, USE_TEST_DATA);

    // Get the grid size
    let x_size : usize = input[0].len();
    let y_size : usize = input.len();

    // Testing the idea - add empty string padding around the grid to avoid having to check for out-of-bound
    // let y_size_padded : usize = y_size + 2;
    // let x_size_padded : usize = x_size + 2;

    //println!("Data size: {}x{}, Enlarged: {}x{}", y_size, x_size, y_size_padded, x_size_padded);

    // Create grid as 2d collection
    let mut grid_raw = vec![-1; 52 * 52];
    let mut grid_base : Vec<_> = grid_raw.as_mut_slice().chunks_mut(52).collect();
    let grid = grid_base.as_mut_slice();

    let mut trailhead_locations : Vec<(usize, usize)> = vec![];

    // Fill the grid, but leave outer padding
    for y in 0..y_size {
        let line : Vec<char> = input[y].chars().collect();
        for x in 0..x_size {
            grid[y + 1][x + 1] = line[x].to_string().parse().expect("Failed to parse.");
            if grid[y + 1][x + 1] == 0 {
                trailhead_locations.push((y + 1, x + 1));
            }
        }
    }

    //print_grid(grid, y_size_padded, x_size_padded);

    let mut total_trailhead_score : usize = 0;

    for tl in trailhead_locations {
        //println!("Trailhead location: y:{},x:{}", tl.0, tl.1);
        let result = traverse(grid, tl.0, tl.1);
        total_trailhead_score += result.len();
        //println!("Trailhead score: {}", result.len());
    }

    total_trailhead_score.to_string()
}

#[allow(dead_code)]
pub fn part_2() -> String
{
    let input = get_input_lines(DAY_STRING, USE_TEST_DATA);

    // Get the grid size
    let x_size : usize = input[0].len();
    let y_size : usize = input.len();

    // Testing the idea - add empty string padding around the grid to avoid having to check for out-of-bound
    // let y_size_padded : usize = y_size + 2;
    // let x_size_padded : usize = x_size + 2;

    // Create grid as 2d collection
    let mut grid_raw = vec![-1; 52 * 52];
    let mut grid_base : Vec<_> = grid_raw.as_mut_slice().chunks_mut(52).collect();
    let grid = grid_base.as_mut_slice();

    let mut trailhead_locations : Vec<(usize, usize)> = vec![];

    // Fill the grid, but leave outer padding
    for y in 0..y_size {
        let line : Vec<char> = input[y].chars().collect();
        for x in 0..x_size {
            grid[y + 1][x + 1] = line[x].to_string().parse().expect("Failed to parse.");
            if grid[y + 1][x + 1] == 0 {
                trailhead_locations.push((y + 1, x + 1));
            }
        }
    }

    //print_grid(grid, y_size_padded, x_size_padded);

    let mut total_trailhead_score : usize = 0;

    for tl in trailhead_locations {
        let result = traverse_dup(grid, tl.0, tl.1);
        total_trailhead_score += result.len();
    }

    total_trailhead_score.to_string()
}

#[allow(dead_code)]
fn print_grid(grid : &mut[&mut[char]], x_size : usize, y_size : usize) {
    for y in 0..y_size {
        for x in 0..x_size {
            print!("{}", grid[y][x]);
        }
        println!("");
    }
}
