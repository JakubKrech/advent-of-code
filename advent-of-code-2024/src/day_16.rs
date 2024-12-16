use std::i32;

use crate::utilities::{self, get_input_lines};

// https://adventofcode.com/2024/day/16

const DAY_STRING : &str = "day_16";
const USE_TEST_DATA : bool = true;

enum DirectionIndex {
    UP = 1,
    RIGHT = 2,
    DOWN = 3,
    LEFT = 4
}

fn walk_grid(grid : &mut[&mut[(char, i32, i32, i32, i32)]], cur_y : usize, cur_x : usize, dir : char, end_y : usize, end_x : usize, cost : i32) {

    if cur_x == end_x && cur_y == end_y {
        println!("End reached, cost: {}", cost);
        return;
    }

    let dir_costs : (i32, i32, i32, i32);

    if dir == '^' || dir == 'v' {
        dir_costs = (1, 1000, 1, 1000); // ^ > v <
    } else {
        dir_costs = (1000, 1, 1000, 1); // ^ > v <
    }

    // ^
    if dir != 'v' && grid[cur_y - 1][cur_x].0 != '#' && grid[cur_y - 1][cur_x].1 > cost + dir_costs.0 {
        walk_grid(grid, cur_y - 1, cur_x, '^', end_y, end_x, dir_costs.0);
    }

    // >
    if dir != '<' && grid[cur_y][cur_x + 1].0 != '#' && grid[cur_y][cur_x + 1].2 > cost + 1 {
        walk_grid(grid, cur_y - 1, cur_x, '^', end_y, end_x, dir_costs.0);
    }


    // v



    // <

    if dir == '^' {
        if grid[cur_y - 1][cur_x].0 != '#' && grid[cur_y - 1][cur_x].1 > cost + 1 {
            walk_grid(grid, cur_y - 1, cur_x, '^', end_y, end_x, cost + 1);
        }
    } else if dir == '<' {
        
    } else if dir == '>' {
        
    }

    
    if grid[cur_y][cur_x + 1].0 != '#' {

        if dir == '>' {

        } else if dir == '^' || dir == 'v' {
            
        }

    }

    // v
    if grid[cur_y + 1][cur_x].0 != '#' {

        if dir == 'v' {

        } else if dir == '<' || dir == '>'  {
            
        }

    }

    // <
    if grid[cur_y][cur_x - 1].0 != '#' {

        if dir == '<' {
        
        } else if dir == '^' || dir == 'v' {
    
        }

    }
}

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
    let mut grid_raw : Vec<(char, i32, i32, i32, i32)> = vec![('.', 0, 0, 0, 0); x_size * y_size];
    let mut grid_base : Vec<_> = grid_raw.as_mut_slice().chunks_mut(x_size).collect();
    let grid = grid_base.as_mut_slice();

    // Fill the grid
    for y in 0..y_size {
        for x in 0..x_size {
            let ch = input[y].chars().nth(x).expect("Failed to fill the grid");
            let digit : i32;

            if ch == '#' {
                digit = -1;
            } else {
                digit = i32::MAX;
            }

            grid[y][x] = (ch, digit, digit, digit, digit);
        }
    }

    println!("s: {}, e: {}", grid[s_y][s_x].0, grid[e_y][e_x].0);

    print_grid_dimension(grid, y_size, x_size, 1);

    return input.len().to_string();
}

#[allow(dead_code)]
pub fn part_2() -> String
{
    let input = get_input_lines(DAY_STRING, USE_TEST_DATA);

    return input.len().to_string();
}

pub fn print_grid_dimension(grid : &mut[&mut[(char, i32, i32, i32, i32)]], y_size : usize, x_size : usize, dimension : usize) {
    for y in 0..y_size {
        for x in 0..x_size {

            if dimension == 0 {
                print!("{:2}", grid[y][x].0);
                continue;
            }

            let to_print = match dimension {
                1 => grid[y][x].1,
                2 => grid[y][x].2,
                3 => grid[y][x].3,
                4 => grid[y][x].4,
                _ => 0
            };

            print!("{:2}", to_print);
        }
        println!("");
    }
}
