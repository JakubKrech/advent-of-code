use std::collections::HashSet;

use crate::utilities::get_input_lines;

// https://adventofcode.com/2024/day/6

const DAY_STRING : &str = "day_06";
const USE_TEST_DATA : bool = false;

pub fn part_1() -> String
{
    let input = get_input_lines(DAY_STRING, USE_TEST_DATA);

    let mut cur_x: usize = 0;
    let mut cur_y: usize = 0;
    let mut cur_rot = '^';

    // Get the grid size
    let x_size : usize = input[0].len();
    let y_size : usize = input.len();

    // Create grid as 2d collection
    let mut grid_raw = vec![' '; x_size * y_size];
    let mut grid_base : Vec<_> = grid_raw.as_mut_slice().chunks_mut(x_size).collect();
    let grid = grid_base.as_mut_slice();

    // Fill grid with data and save X locations
    for y in 0..y_size {
        for x in 0..x_size {
            grid[y][x] = input[y].chars().nth(x).unwrap();

            if grid[y][x] == '^' {
                cur_x = x;
                cur_y = y;
            }
        }
    }

    //print_grid(grid);

    // Move and paint the floor until leaving the board
    loop {
        grid[cur_y][cur_x] = 'X';

        // moving up
        if cur_rot == '^' {
            if cur_y == 0 { break; }

            if grid[cur_y - 1][cur_x] == '#' {
                cur_rot = '>';
            }
            else {
                cur_y -= 1;
            }
        }

        // moving right
        else if cur_rot == '>' {
            if cur_x == x_size - 1 { break; }

            if grid[cur_y][cur_x + 1] == '#' {
                cur_rot = 'v';
            }
            else {
                cur_x += 1;
            }
        } 

        // moving down
        if cur_rot == 'v' {
            if cur_y == y_size - 1 { break; }

            if grid[cur_y + 1][cur_x] == '#' {
                cur_rot = '<';
            }
            else {
                cur_y += 1;
            }
        }

        // moving left
        if cur_rot == '<' {
            if cur_x == 0 { break; }

            if grid[cur_y][cur_x - 1] == '#' {
                cur_rot = '^';
            }
            else {
                cur_x -= 1;
            }
        }
    }

    //print_grid(grid);

    let mut result : i32 = 0;

    for line in grid.iter() {
        for c in line.iter() {
            if *c == 'X' {
                result += 1;
            }
        }
    }

    return result.to_string();
}

pub fn part_2() -> String
{
    let input = get_input_lines(DAY_STRING, USE_TEST_DATA);

    let mut start_x: usize = 0;
    let mut start_y: usize = 0;
    let mut cur_x: usize = 0;
    let mut cur_y: usize = 0;
    let mut cur_rot = '^';
    let mut epoch : usize = 0;

    // Get the grid size
    let x_size : usize = input[0].len();
    let y_size : usize = input.len();

    // Create grid as 2d collection
    let mut grid_raw = vec![' '; x_size * y_size];
    let mut grid_base : Vec<_> = grid_raw.as_mut_slice().chunks_mut(x_size).collect();
    let grid = grid_base.as_mut_slice();

    // Fill grid with data and save X locations
    for y in 0..y_size {
        for x in 0..x_size {
            grid[y][x] = input[y].chars().nth(x).unwrap();

            if grid[y][x] == '^' {
                start_y = y;
                start_x = x;
                cur_x = x;
                cur_y = y;
            }
        }
    }

    let mut guard_locations : Vec<(usize, usize)> = vec![];

    // Move and paint the floor until leaving the board
    loop {
        epoch += 1;
        grid[cur_y][cur_x] = 'X';
        guard_locations.push((cur_y, cur_x));

        // moving up
        if cur_rot == '^' {
            if cur_y == 0 { break; }

            if grid[cur_y - 1][cur_x] == '#' {
                cur_rot = '>';
            }
            else {
                cur_y -= 1;
            }
        }

        // moving right
        else if cur_rot == '>' {
            if cur_x == x_size - 1 { break; }

            if grid[cur_y][cur_x + 1] == '#' {
                cur_rot = 'v';
            }
            else {
                cur_x += 1;
            }
        } 

        // moving down
        if cur_rot == 'v' {
            if cur_y == y_size - 1 { break; }

            if grid[cur_y + 1][cur_x] == '#' {
                cur_rot = '<';
            }
            else {
                cur_y += 1;
            }
        }

        // moving left
        if cur_rot == '<' {
            if cur_x == 0 { break; }

            if grid[cur_y][cur_x - 1] == '#' {
                cur_rot = '^';
            }
            else {
                cur_x -= 1;
            }
        }
    }

    let unique_guards_locations : HashSet<(usize, usize)> = guard_locations.iter().cloned().collect();

    let mut obstruction_y : usize = 0;
    let mut obstruction_x : usize = 0;
    let mut result : i32 = 0;
    
    // Following guard path check every position for possible loop creating obstruction
    for (y, x) in unique_guards_locations {
        // Reset grid
        reset_grid(grid, y_size, x_size);
        cur_x = start_x;
        cur_y = start_y;
        cur_rot = '^';
        grid[obstruction_y][obstruction_x] = '.';

        // Place the obstruction
        obstruction_y = y;
        obstruction_x = x;
        grid[obstruction_y][obstruction_x] = '#';

        epoch = 0;

        // Move around until unreasonable number of epochs happens, meaning we are stuck in a loop.
        // This could be improved by some smart loop detection.
        loop {
            if epoch > 10000 {
                result += 1;
                break;
            }

            epoch += 1;
            grid[cur_y][cur_x] = 'X';

            // moving up
            if cur_rot == '^' {
                if cur_y == 0 { break; }

                if grid[cur_y - 1][cur_x] == '#' {
                    cur_rot = '>';
                }
                else {
                    cur_y -= 1;
                }
            }

            // moving right
            else if cur_rot == '>' {
                if cur_x == x_size - 1 { break; }

                if grid[cur_y][cur_x + 1] == '#' {
                    cur_rot = 'v';
                }
                else {
                    cur_x += 1;
                }
            } 

            // moving down
            if cur_rot == 'v' {
                if cur_y == y_size - 1 { break; }

                if grid[cur_y + 1][cur_x] == '#' {
                    cur_rot = '<';
                }
                else {
                    cur_y += 1;
                }
            }

            // moving left
            if cur_rot == '<' {
                if cur_x == 0 { break; }

                if grid[cur_y][cur_x - 1] == '#' {
                    cur_rot = '^';
                }
                else {
                    cur_x -= 1;
                }
            }
        }
    } 

    return result.to_string();
}

// fn print_grid(grid : &mut [&mut [ char]]) {
//     println!("");
//     for line in grid.iter() {
//         for c in line.iter() {
//             print!("{}", c);
//         }
//         println!("");
//     }
// }

fn reset_grid(grid : &mut [&mut [ char]], y_size : usize, x_size : usize) {
    for y in 0..y_size {
        for x in 0..x_size {
            if grid[y][x] == 'X' { grid[y][x] = '.'; }
        }
    }
}