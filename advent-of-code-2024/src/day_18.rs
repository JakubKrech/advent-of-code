use std::{collections::VecDeque, usize};

use crate::utilities::get_input_lines;

// https://adventofcode.com/2024/day/18

const DAY_STRING : &str = "day_18";
const USE_TEST_DATA : bool = false;

const GRID_SIZE : usize = 71;

fn pathfind(how_many_bytes : usize) -> usize {
    
    let input = get_input_lines(DAY_STRING, USE_TEST_DATA);

    let start_y : usize = 0;
    let start_x : usize = 0;

    let mut grid_raw : Vec<char> = vec!['.'; GRID_SIZE * GRID_SIZE];
    let mut grid_base : Vec<_> = grid_raw.as_mut_slice().chunks_mut(GRID_SIZE).collect();
    let grid = grid_base.as_mut_slice();

    let mut grid_raw : Vec<usize> = vec![usize::MAX; GRID_SIZE * GRID_SIZE];
    let mut grid_base : Vec<_> = grid_raw.as_mut_slice().chunks_mut(GRID_SIZE).collect();
    let grid_costs = grid_base.as_mut_slice();

    grid_costs[start_y][start_x] = 0;

    // Read input and mark grid
    for i in 0..how_many_bytes {
        let data : Vec<&str> = input[i].split_terminator(',').collect();

        let y : usize = data[1].parse().expect("Failed to parse");
        let x : usize = data[0].parse().expect("Failed to parse");

        grid[y][x] = '#';
        grid_costs[y][x] = 0;
    }

    let start_y : usize = 0;
    let start_x : usize = 0;

    let end_y : usize = GRID_SIZE - 1;
    let end_x : usize = GRID_SIZE - 1;
    
    let mut queue : VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_front((start_y, start_x));

    while queue.len() > 0 {

        let current = queue.pop_back().expect("Failed to pop");

        // ^
        if current.0 > 0 {
            let target = (current.0 - 1, current.1);

            if grid[target.0][target.1] != '#' && grid_costs[target.0][target.1] > grid_costs[current.0][current.1] + 1 {
                grid_costs[target.0][target.1] = grid_costs[current.0][current.1] + 1;
                queue.push_front(target);
            }
        }

        // >
        if current.1 + 1 < GRID_SIZE {
            let target = (current.0, current.1 + 1);

            if grid[target.0][target.1] != '#' && grid_costs[target.0][target.1] > grid_costs[current.0][current.1] + 1 {
                grid_costs[target.0][target.1] = grid_costs[current.0][current.1] + 1;
                queue.push_front(target);
            }
        }

        // v
        if current.0 + 1 < GRID_SIZE {
            let target = (current.0 + 1, current.1);

            if grid[target.0][target.1] != '#' && grid_costs[target.0][target.1] > grid_costs[current.0][current.1] + 1 {
                grid_costs[target.0][target.1] = grid_costs[current.0][current.1] + 1;
                queue.push_front(target);
            }
        }

        // <
        if current.1 > 0 {
            let target = (current.0, current.1 - 1);

            if grid[target.0][target.1] != '#' && grid_costs[target.0][target.1] > grid_costs[current.0][current.1] + 1 {
                grid_costs[target.0][target.1] = grid_costs[current.0][current.1] + 1;
                queue.push_front(target);
            }
        }
    }

    return grid_costs[end_y][end_x];
}

#[allow(dead_code)]
pub fn part_1() -> String
{
    let mut byte_count : usize = 1024;

    if USE_TEST_DATA {
        byte_count = 7;
    }

    let result : usize = pathfind(byte_count);

    return result.to_string();
}

#[allow(dead_code)]
pub fn part_2() -> String
{
    let input = get_input_lines(DAY_STRING, USE_TEST_DATA);

    let mut how_many_bytes_from : usize = 0;
    let mut how_many_bytes_to : usize = input.len() - 1;

    while how_many_bytes_to - how_many_bytes_from > 1 {

        let check_byte_count = (how_many_bytes_to + how_many_bytes_from) / 2;
        let result : usize = pathfind(check_byte_count);

        if result == usize::MAX {
            how_many_bytes_to = check_byte_count;
        } else {
            how_many_bytes_from = check_byte_count;
        }
    }

    return input[how_many_bytes_from].to_string();
}

#[allow(dead_code)]
pub fn print_grid_costs(grid : &mut[&mut[usize]], y_size : usize, x_size : usize) {
    for y in 0..y_size {
        for x in 0..x_size {
            print!("{} ", grid[y][x]);
        }
        println!("");
    }
}