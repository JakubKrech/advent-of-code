use std::collections::HashMap;
use std::collections::HashSet;

use crate::utilities::get_input_lines;

// https://adventofcode.com/2024/day/8

const DAY_STRING : &str = "day_08";
const USE_TEST_DATA : bool = false;

#[allow(dead_code)]
pub fn part_1() -> String
{
    let input = get_input_lines(DAY_STRING, USE_TEST_DATA);

    let mut antennas_locations : HashMap<char, Vec<(i32, i32)>> = HashMap::new();

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

            if grid[y][x] != '.' {
                let yy : i32 = y.try_into().expect("Failed to parse");
                let xx : i32 = x.try_into().expect("Failed to parse");
                if antennas_locations.contains_key(&grid[y][x]) {
                    antennas_locations.get_mut(&grid[y][x]).expect("Failed to get mut").push((yy, xx));
                } else {
                    antennas_locations.insert(grid[y][x].clone(), vec![(yy, xx)]);
                }
            }
        }
    }

    // print_grid(grid, x_size, y_size);

    let mut unique_antinodes_locations : HashSet<(i32, i32)> = HashSet::new();

    for antenna_type_info in antennas_locations {
        let antenna_count = &antenna_type_info.1.len();
        
        // For each antenna combination calculate if their antinodes are in bounds of the board
        for i in 0..*antenna_count {
            for j in i+1..*antenna_count {
                // Get distance between antennas
                let dy : i32 = &antenna_type_info.1[j].0 -  &antenna_type_info.1[i].0;
                let dx : i32 = &antenna_type_info.1[j].1 -  &antenna_type_info.1[i].1;

                // Add deltas to antenna #2
                let antinode_2_y = &antenna_type_info.1[j].0 + dy;
                let antinode_2_x = &antenna_type_info.1[j].1 + dx;

                // Substract deltas from antenna #1
                let antinode_1_y = &antenna_type_info.1[i].0 - dy;
                let antinode_1_x = &antenna_type_info.1[i].1 - dx;

                // Check if antinodes are inside grid
                if antinode_2_y >= 0 && antinode_2_y < y_size.try_into().expect("Fail") && antinode_2_x >= 0 && antinode_2_x < x_size.try_into().expect("Fail") {
                    unique_antinodes_locations.insert((antinode_2_y, antinode_2_x));
                }

                if antinode_1_y >= 0 && antinode_1_y < y_size.try_into().expect("Fail") && antinode_1_x >= 0 && antinode_1_x < x_size.try_into().expect("Fail") {
                    unique_antinodes_locations.insert((antinode_1_y, antinode_1_x));
                }
            }
        }
    }

    return unique_antinodes_locations.len().to_string();
}

#[allow(dead_code)]
pub fn part_2() -> String
{
    let input = get_input_lines(DAY_STRING, USE_TEST_DATA);

    let mut antennas_locations : HashMap<char, Vec<(i32, i32)>> = HashMap::new();

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

            if grid[y][x] != '.' {
                let yy : i32 = y.try_into().expect("Failed to parse");
                let xx : i32 = x.try_into().expect("Failed to parse");
                if antennas_locations.contains_key(&grid[y][x]) {
                    antennas_locations.get_mut(&grid[y][x]).expect("Failed to get mut").push((yy, xx));
                } else {
                    antennas_locations.insert(grid[y][x].clone(), vec![(yy, xx)]);
                }
            }
        }
    }

    // print_grid(grid, x_size, y_size);

    let mut unique_antinodes_locations : HashSet<(i32, i32)> = HashSet::new();

    for antenna_type_info in antennas_locations {
        let antenna_count = &antenna_type_info.1.len();
        
        // For each antenna combination calculate if their antinodes are in bounds of the board
        for i in 0..*antenna_count {
            for j in i+1..*antenna_count {

                // Antennas are antinodes now, count them in
                unique_antinodes_locations.insert((antenna_type_info.1[i].0, antenna_type_info.1[i].1));
                unique_antinodes_locations.insert((antenna_type_info.1[j].0, antenna_type_info.1[j].1));

                // Get distance between antennas
                let dy : i32 = &antenna_type_info.1[j].0 -  &antenna_type_info.1[i].0;
                let dx : i32 = &antenna_type_info.1[j].1 -  &antenna_type_info.1[i].1;

                // Add deltas to series of antennas, starting from antenna #2
                let mut antinode_2_y = antenna_type_info.1[j].0.clone();
                let mut antinode_2_x = antenna_type_info.1[j].1.clone();

                while antinode_2_y + dy >= 0 && antinode_2_y + dy < y_size.try_into().expect("Fail") && antinode_2_x + dx >= 0 && antinode_2_x + dx < x_size.try_into().expect("Fail") {
                    antinode_2_y += dy;
                    antinode_2_x += dx;
                    unique_antinodes_locations.insert((antinode_2_y, antinode_2_x));
                }
                
                // Substract deltas from series of inside antennas, starting from antenna #1
                let mut antinode_1_y = antenna_type_info.1[i].0.clone();
                let mut antinode_1_x = antenna_type_info.1[i].1.clone();

                while antinode_1_y - dy >= 0 && antinode_1_y - dy < y_size.try_into().expect("Fail") && antinode_1_x - dx >= 0 && antinode_1_x - dx < x_size.try_into().expect("Fail") {
                    antinode_1_y -= dy;
                    antinode_1_x -= dx;
                    unique_antinodes_locations.insert((antinode_1_y, antinode_1_x));
                }
            }
        }
    }

    return unique_antinodes_locations.len().to_string();
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
