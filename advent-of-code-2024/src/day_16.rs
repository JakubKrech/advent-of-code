use std::{collections::{VecDeque, HashSet}, usize};

use crate::utilities::get_input_lines;

// https://adventofcode.com/2024/day/16

const DAY_STRING : &str = "day_16";
const USE_TEST_DATA : bool = false;

#[allow(dead_code)]
pub fn part_1() -> String
{
    let input = get_input_lines(DAY_STRING, USE_TEST_DATA);

    let y_size : usize = input.len();
    let x_size : usize = input[0].len();

    let s_y : usize = y_size - 2;
    let s_x : usize = 1;

    let e_y : usize = 1;
    let e_x : usize = x_size - 2;

    // Create grid as 2d collection
    let mut grid_raw : Vec<(char, usize, usize, usize, usize)> = vec![('.', usize::MAX, usize::MAX, usize::MAX, usize::MAX); x_size * y_size];
    let mut grid_base : Vec<_> = grid_raw.as_mut_slice().chunks_mut(x_size).collect();
    let grid = grid_base.as_mut_slice();

    // Fill the grid
    for y in 0..y_size {
        for x in 0..x_size {
            let ch = input[y].chars().nth(x).expect("Failed to fill the grid");
            grid[y][x].0 = ch;

            if grid[y][x].0 == '#' || grid[y][x].0 == 'S' {
                grid[y][x].1 = 0; grid[y][x].2 = 0; grid[y][x].3 = 0; grid[y][x].4 = 0;
            }
        }
    }

    //                             cur_y, cur_x, curr_cost, coming_from_dir
    let mut processing_queue : VecDeque<(usize, usize, usize, char)> = VecDeque::new();
    processing_queue.push_front((s_y, s_x, 0, '>'));

    while !processing_queue.is_empty() {

        let curr = processing_queue.pop_back().unwrap();
        let cur_y = curr.0;
        let cur_x = curr.1;
        let cur_cost = curr.2;
        let from_dir = curr.3;

        // try ^
        if from_dir != 'v' {
            let mut move_cost : usize = 1;
            if from_dir != '^' { move_cost += 1000 };
            if grid[cur_y - 1][cur_x].1 > cur_cost + move_cost {
                grid[cur_y - 1][cur_x].1 = cur_cost + move_cost;
                processing_queue.push_front((cur_y - 1, cur_x, cur_cost + move_cost, '^'));
            }
        }

        // try >
        if from_dir != '<' {
            let mut move_cost : usize  = 1;
            if from_dir != '>' { move_cost += 1000 };
            if grid[cur_y][cur_x + 1].2 > cur_cost + move_cost {
                grid[cur_y][cur_x + 1].2 = cur_cost + move_cost;
                processing_queue.push_front((cur_y, cur_x + 1, cur_cost + move_cost, '>'));
            }
        }

        // try v
        if from_dir != '^' {
            let mut move_cost : usize  = 1;
            if from_dir != 'v' { move_cost += 1000 };
            if grid[cur_y + 1][cur_x].3 > cur_cost + move_cost {
                grid[cur_y + 1][cur_x].3 = cur_cost + move_cost;
                processing_queue.push_front((cur_y + 1, cur_x, cur_cost + move_cost, 'v'));
            }
        }

        // try <
        if from_dir != '>' {
            let mut move_cost : usize  = 1;
            if from_dir != '<' { move_cost += 1000 };
            if grid[cur_y][cur_x - 1].4 > cur_cost + move_cost {
                grid[cur_y][cur_x - 1].4 = cur_cost + move_cost;
                processing_queue.push_front((cur_y, cur_x - 1, cur_cost + move_cost, '<'));
            }
        }
    }

    let mut result : usize = usize::MAX;

    if grid[e_y][e_x].1 < result { result = grid[e_y][e_x].1; }
    if grid[e_y][e_x].2 < result { result = grid[e_y][e_x].2; }
    if grid[e_y][e_x].3 < result { result = grid[e_y][e_x].3; }
    if grid[e_y][e_x].4 < result { result = grid[e_y][e_x].4; }

    return result.to_string();
}



#[allow(dead_code)]
pub fn part_2() -> String
{
    let input = get_input_lines(DAY_STRING, USE_TEST_DATA);

    let y_size : usize = input.len();
    let x_size : usize = input[0].len();

    let s_y : usize = y_size - 2;
    let s_x : usize = 1;

    // Create grid as 2d collection
    let mut grid_raw : Vec<(char, usize, usize, usize, usize)> = vec![('.', usize::MAX, usize::MAX, usize::MAX, usize::MAX); x_size * y_size];
    let mut grid_base : Vec<_> = grid_raw.as_mut_slice().chunks_mut(x_size).collect();
    let grid = grid_base.as_mut_slice();

    // Fill the grid
    for y in 0..y_size {
        for x in 0..x_size {
            let ch = input[y].chars().nth(x).expect("Failed to fill the grid");
            grid[y][x].0 = ch;

            if grid[y][x].0 == '#' || grid[y][x].0 == 'S' {
                grid[y][x].1 = 0; grid[y][x].2 = 0; grid[y][x].3 = 0; grid[y][x].4 = 0;
            }
        }
    }

    //                             cur_y, cur_x, curr_cost, coming_from_dir
    let mut processing_queue : VecDeque<(usize, usize, usize, char)> = VecDeque::new();
    processing_queue.push_front((s_y, s_x, 0, '>'));

    while !processing_queue.is_empty() {

        let curr = processing_queue.pop_back().unwrap();
        let cur_y = curr.0;
        let cur_x = curr.1;
        let cur_cost = curr.2;
        let from_dir = curr.3;

        // try ^
        if from_dir != 'v' {
            let mut move_cost : usize = 1;
            if from_dir != '^' { move_cost += 1000 };
            if grid[cur_y - 1][cur_x].1 > cur_cost + move_cost {
                grid[cur_y - 1][cur_x].1 = cur_cost + move_cost;
                processing_queue.push_front((cur_y - 1, cur_x, cur_cost + move_cost, '^'));
            }
        }

        // try >
        if from_dir != '<' {
            let mut move_cost : usize  = 1;
            if from_dir != '>' { move_cost += 1000 };
            if grid[cur_y][cur_x + 1].2 > cur_cost + move_cost {
                grid[cur_y][cur_x + 1].2 = cur_cost + move_cost;
                processing_queue.push_front((cur_y, cur_x + 1, cur_cost + move_cost, '>'));
            }
        }

        // try v
        if from_dir != '^' {
            let mut move_cost : usize  = 1;
            if from_dir != 'v' { move_cost += 1000 };
            if grid[cur_y + 1][cur_x].3 > cur_cost + move_cost {
                grid[cur_y + 1][cur_x].3 = cur_cost + move_cost;
                processing_queue.push_front((cur_y + 1, cur_x, cur_cost + move_cost, 'v'));
            }
        }

        // try <
        if from_dir != '>' {
            let mut move_cost : usize  = 1;
            if from_dir != '<' { move_cost += 1000 };
            if grid[cur_y][cur_x - 1].4 > cur_cost + move_cost {
                grid[cur_y][cur_x - 1].4 = cur_cost + move_cost;
                processing_queue.push_front((cur_y, cur_x - 1, cur_cost + move_cost, '<'));
            }
        }
    }

    let mut best_spots_coords : HashSet<(usize, usize)> = HashSet::new();
    walk_optimal_and_save_nodes(grid, s_y, s_x, '>', &mut best_spots_coords);

    return best_spots_coords.len().to_string();
}

fn walk_optimal_and_save_nodes(grid : &mut[&mut[(char, usize, usize, usize, usize)]], cur_y : usize, cur_x : usize, from_dir : char, best_spots_coords : &mut HashSet<(usize, usize)>) -> bool {
    let cur_cost : usize = match from_dir {
        '^' => grid[cur_y][cur_x].1,
        '>' => grid[cur_y][cur_x].2,
        'v' => grid[cur_y][cur_x].3,
        '<' => grid[cur_y][cur_x].4,
        _ => 0
    };

    if cur_y == 1 && cur_x == grid[0].len() - 2 {
        if calculate_min_cell_cost(grid, cur_y, cur_x) == cur_cost {
            best_spots_coords.insert((cur_y, cur_x));
            return true;
        } else {
            return false;
        }
    }

    let mut optimal_walk_succeeded : bool = false;

    // Try walk ^
    if from_dir != 'v' {
        let mut move_cost : usize = 1;
        if from_dir != '^' { move_cost += 1000 };
        if grid[cur_y - 1][cur_x].1 == cur_cost + move_cost {
            let reached_optimally = walk_optimal_and_save_nodes(grid, cur_y - 1, cur_x, '^', best_spots_coords);
            if reached_optimally {
                best_spots_coords.insert((cur_y, cur_x));
                optimal_walk_succeeded = true;
            }
        }
    }

    // Try walk >
    if from_dir != '<' {
        let mut move_cost : usize = 1;
        if from_dir != '>' { move_cost += 1000 };
        if grid[cur_y][cur_x + 1].2 == cur_cost + move_cost {
            let reached_optimally = walk_optimal_and_save_nodes(grid, cur_y, cur_x + 1, '>', best_spots_coords);
            if reached_optimally {
                best_spots_coords.insert((cur_y, cur_x));
                optimal_walk_succeeded = true;
            }
        }
    }

    // Try walk v
    if from_dir != '^' {
        let mut move_cost : usize = 1;
        if from_dir != 'v' { move_cost += 1000 };
        if grid[cur_y + 1][cur_x].3 == cur_cost + move_cost {
            let reached_optimally = walk_optimal_and_save_nodes(grid, cur_y + 1, cur_x, 'v', best_spots_coords);
            if reached_optimally {
                best_spots_coords.insert((cur_y, cur_x));
                optimal_walk_succeeded = true;
            }
        }
    }
    
    // Try walk <
    if from_dir != '>' {
        let mut move_cost : usize = 1;
        if from_dir != '<' { move_cost += 1000 };
        if grid[cur_y][cur_x - 1].4 == cur_cost + move_cost {
            let reached_optimally = walk_optimal_and_save_nodes(grid, cur_y, cur_x - 1, '<', best_spots_coords);
            if reached_optimally {
                best_spots_coords.insert((cur_y, cur_x));
                optimal_walk_succeeded = true;
            }
        }
    }

    return optimal_walk_succeeded;
}

fn calculate_min_cell_cost(grid : &mut[&mut[(char, usize, usize, usize, usize)]], yy : usize, xx : usize) -> usize {
    let mut min_cost : usize = usize::MAX;
    if grid[yy][xx].1 < min_cost { min_cost = grid[yy][xx].1; }
    if grid[yy][xx].2 < min_cost { min_cost = grid[yy][xx].2; }
    if grid[yy][xx].3 < min_cost { min_cost = grid[yy][xx].3; }
    if grid[yy][xx].4 < min_cost { min_cost = grid[yy][xx].4; }
    return min_cost;
}
