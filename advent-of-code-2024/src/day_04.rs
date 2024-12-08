use crate::utilities::get_input_lines;

// https://adventofcode.com/2024/day/4

// XY AXIS y
//         y
//         y
// xxxxxxxxoxxxxxxxxx
//         y
//         y
//         y

const DAY_STRING : &str = "day_04";
const USE_TEST_DATA : bool = false;

#[allow(dead_code)]
pub fn part_1() -> String
{
    let input = get_input_lines(DAY_STRING, USE_TEST_DATA);
    let mut x_locations : Vec<(usize, usize)> = vec![];
    let mut xmas_counter : i32 = 0; 

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

            if grid[y][x] == 'X' {
                x_locations.push((y, x));
            }
        }
    }

    // print_grid(grid, x_size, y_size);

    // For each saved X location attempt to find XMAS in every direction - N, E, S, W, NE, SE, SW, NW
    // For XMAS to be possible we need to be able to move 3 times in given direction.
    for location in x_locations {
        let y_u = location.0;
        let x_u = location.1;
        let y : i32 = i32::try_from(y_u).expect("Failed to parse usize to i32");
        let x : i32 = i32::try_from(x_u).expect("Failed to parse usize to i32");

        // Look N ^
        if y - 3 >= 0 {
            if grid[y_u - 1][x_u] == 'M' && grid[y_u - 2][x_u] == 'A' && grid[y_u - 3][x_u] == 'S' {
                //println!("FOUND {},{} ^", y_u, x_u);
                xmas_counter += 1;
            }
        }

        // Look E >
        if x_u + 3 < x_size {
            if grid[y_u][x_u + 1] == 'M' && grid[y_u][x_u + 2] == 'A' && grid[y_u][x_u + 3] == 'S' {
                //println!("FOUND {},{} >", y_u, x_u);
                xmas_counter += 1;
            }
        }

        // Look S v
        if y_u + 3 < y_size {
            if grid[y_u + 1][x_u] == 'M' && grid[y_u + 2][x_u] == 'A' && grid[y_u + 3][x_u] == 'S' {
                //println!("FOUND {},{} v", y_u, x_u);
                xmas_counter += 1;
            }
        }

        // Look W <
        if x - 3 >= 0 {
            if grid[y_u][x_u - 1] == 'M' && grid[y_u][x_u - 2] == 'A' && grid[y_u][x_u - 3] == 'S' {
                //println!("FOUND {},{} <", y_u, x_u);
                xmas_counter += 1;
            }
        }

        // Look NE />
        if y - 3 >= 0 && x_u + 3 < x_size {
            if grid[y_u - 1][x_u + 1] == 'M' && grid[y_u - 2][x_u + 2] == 'A' && grid[y_u - 3][x_u + 3] == 'S' {
                //println!("FOUND {},{} />", y_u, x_u);
                xmas_counter += 1;
            }
        }

        // Look SE \>
        if y_u + 3 < y_size && x_u + 3 < x_size {
            if grid[y_u + 1][x_u + 1] == 'M' && grid[y_u + 2][x_u + 2] == 'A' && grid[y_u + 3][x_u + 3] == 'S' {
                //println!("FOUND {},{} \\>", y_u, x_u);
                xmas_counter += 1;
            }
        }

        // Look SW /<
        if y_u + 3 < y_size && x - 3 >= 0 {
            if grid[y_u + 1][x_u - 1] == 'M' && grid[y_u + 2][x_u - 2] == 'A' && grid[y_u + 3][x_u - 3] == 'S' {
                //println!("FOUND {},{} /<", y_u, x_u);
                xmas_counter += 1;
            }
        }

        // Look NW \<
        if y - 3 >= 0 && x - 3 >= 0 {
            if grid[y_u - 1][x_u - 1] == 'M' && grid[y_u - 2][x_u - 2] == 'A' && grid[y_u - 3][x_u - 3] == 'S' {
                //println!("FOUND {},{} \\<", y_u, x_u);
                xmas_counter += 1;
            }
        }
    }

    return xmas_counter.to_string();
}

#[allow(dead_code)]
pub fn part_2() -> String
{
    let input = get_input_lines(DAY_STRING, USE_TEST_DATA);
    let mut x_locations : Vec<(usize, usize)> = vec![];
    let mut xmas_counter : i32 = 0; 

    // Get the grid size
    let x_size : usize = input[0].len();
    let y_size : usize = input.len();
    // println!("Data size: {}x{}", x_size, y_size);

    // Create grid as 2d collection
    let mut grid_raw = vec![' '; x_size * y_size];
    let mut grid_base : Vec<_> = grid_raw.as_mut_slice().chunks_mut(x_size).collect();
    let grid = grid_base.as_mut_slice();

    // println!("Grid size: {}x{}", grid[0].len(), grid.len());

    // Fill grid with data and save A locations
    for y in 0..y_size {
        for x in 0..x_size {
            grid[y][x] = input[y].chars().nth(x).unwrap();

            if grid[y][x] == 'A' {
                x_locations.push((y, x));
            }
        }
    }

    // print_grid(grid, x_size, y_size);

    // For each saved A location attempt to find X-MAS in every possible combination
    for location in x_locations {
        let y_u = location.0;
        let x_u = location.1;
        let y : i32 = i32::try_from(y_u).expect("Failed to parse usize to i32");
        let x : i32 = i32::try_from(x_u).expect("Failed to parse usize to i32");

        // M S
        //  A
        // M S
        if y - 1 >= 0 && x - 1 >= 0  && y_u + 1 < y_size && x_u + 1 < x_size {
            if grid[y_u - 1][x_u - 1] == 'M' && grid[y_u + 1][x_u - 1] == 'M' && grid[y_u + 1][x_u + 1] == 'S' && grid[y_u - 1][x_u + 1] == 'S' {
                // println!("FOUND {},{} LEFT MM", y_u, x_u);
                xmas_counter += 1;
            }
        }

        // S M
        //  A
        // S M
        if y - 1 >= 0 && x - 1 >= 0  && y_u + 1 < y_size && x_u + 1 < x_size {
            if grid[y_u - 1][x_u - 1] == 'S' && grid[y_u + 1][x_u - 1] == 'S' && grid[y_u + 1][x_u + 1] == 'M' && grid[y_u - 1][x_u + 1] == 'M' {
                // println!("FOUND {},{} RIGHT MM", y_u, x_u);
                xmas_counter += 1;
            }
        }

        // S S
        //  A
        // M M
        if y - 1 >= 0 && x - 1 >= 0  && y_u + 1 < y_size && x_u + 1 < x_size {
            if grid[y_u - 1][x_u - 1] == 'S' && grid[y_u + 1][x_u - 1] == 'M' && grid[y_u + 1][x_u + 1] == 'M' && grid[y_u - 1][x_u + 1] == 'S' {
                // println!("FOUND {},{} BOTTOM MM", y_u, x_u);
                xmas_counter += 1;
            }
        }

        // M M
        //  A
        // S S
        if y - 1 >= 0 && x - 1 >= 0  && y_u + 1 < y_size && x_u + 1 < x_size {
            if grid[y_u - 1][x_u - 1] == 'M' && grid[y_u + 1][x_u - 1] == 'S' && grid[y_u + 1][x_u + 1] == 'S' && grid[y_u - 1][x_u + 1] == 'M' {
                // println!("FOUND {},{} UPPER MM", y_u, x_u);
                xmas_counter += 1;
            }
        }
    }

    return xmas_counter.to_string();
}

// fn print_grid(grid : &mut[&mut[char]], x_size : usize, y_size : usize) {
//     for y in 0..y_size {
//         for x in 0..x_size {
//             print!("{}", grid[y][x]);
//         }
//         println!("");
//     }
// }
