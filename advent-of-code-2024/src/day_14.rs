use std::io::prelude::*;
use std::fs::OpenOptions;
use crate::utilities::get_input_lines;

// https://adventofcode.com/2024/day/14

const DAY_STRING : &str = "day_14";
const USE_TEST_DATA : bool = false;

#[allow(dead_code)]
pub fn part_1() -> String
{
    // From exercise description
    // let x_size = 11; // Test input
    // let y_size = 7;  // Test input
    let x_size = 101;
    let y_size = 103;
    let time = 100;

    let input = get_input_lines(DAY_STRING, USE_TEST_DATA);

    let mut locations_after_time : Vec<(i64, i64)> = vec![];

    for line in &input {
        let data_1 : Vec<&str> = line.split_terminator(' ').collect();

        let xy_data : Vec<&str> = data_1[0].split_terminator(',').collect();
        let v_data : Vec<&str> = data_1[1].split_terminator(',').collect();

        let mut x : i64 = xy_data[0][2..].parse().expect("Failed to parse");
        let mut y : i64 = xy_data[1].parse().expect("Failed to parse");

        let dx : i64 = v_data[0][2..].parse().expect("Failed to parse");
        let dy : i64 = v_data[1].parse().expect("Failed to parse");

        for _ in 0..time {
            x += dx;
            y += dy;

            x %= x_size;
            y %= y_size;

            if x < 0 {
                x += x_size;
            }

            if y < 0 {
                y += y_size;
            }
        }

        locations_after_time.push((y, x));
    }

    let first_quarter_count = locations_after_time.clone().into_iter().filter(|x| x.0 < y_size / 2 && x.1 < x_size / 2).count();
    let second_quarter_count = locations_after_time.clone().into_iter().filter(|x| x.0 < y_size / 2 && x.1 > x_size / 2).count();
    let third_quarter_count = &locations_after_time.clone().into_iter().filter(|x| x.0 > y_size / 2 && x.1 < x_size / 2).count();
    let fourth_quarter_count = &locations_after_time.into_iter().filter(|x| x.0 > y_size / 2 && x.1 > x_size / 2).count();
    
    let mutliplied_count = first_quarter_count * second_quarter_count * third_quarter_count * fourth_quarter_count;

    return mutliplied_count.to_string();
}

#[allow(dead_code)]
pub fn part_2() -> String
{
    /*
    
    This part of the exercise was a bit unusual because it while it told us to look for a picture Christmas Tree it did not specify how
    exactly the picture of a Christmas Tree looks like. Maybe the "picture" part was a hint that pictures have frames and that we should
    look for the points forming rectangle frame. But I did not catch that hint, so here is what I did:

    1. First three wrong answers provided to AoC exercise tell you if the result is too high or too low. So I tried to evaluate what is
       the range of the results we are looking for. I tried those three values and found out that range is 5501 - 9999.
        - 1000 -> too low
        - 10000 -> too high
        - 5500 -> too low
    2. Printing 4500 grids in terminal while looking for the Christmas Tree was not the best idea. So instead I wrote all the grid setups
       and annotations about how many seconds elapsed to a single txt file (472500 lines). Then, using VS Code minimap I sprinted through
       the file looking for the Christmas Tree.
    3. Finally I found the picture of Christmas Tree and found out after how many seconds it first appeared!

    */

    // From exercise description
    let x_size = 101; let grid_x_size : usize = 101;
    let y_size = 103; let grid_y_size : usize = 103;

    let input = get_input_lines(DAY_STRING, USE_TEST_DATA);

    // Vec<(y, x, dy, dx)>
    let mut locations_after_time : Vec<(i64, i64, i64, i64)> = vec![];

    // Read initial locations
    for line in &input {
        let data_1 : Vec<&str> = line.split_terminator(' ').collect();

        let xy_data : Vec<&str> = data_1[0].split_terminator(',').collect();
        let v_data : Vec<&str> = data_1[1].split_terminator(',').collect();

        let x : i64 = xy_data[0][2..].parse().expect("Failed to parse");
        let y : i64 = xy_data[1].parse().expect("Failed to parse");

        let dx : i64 = v_data[0][2..].parse().expect("Failed to parse");
        let dy : i64 = v_data[1].parse().expect("Failed to parse");

        locations_after_time.push((y, x, dy, dx));
    }

    let mut time = 0;
    loop {
        time += 1;

        if time >= 10000 {
            break;
        }

        for loc in &mut locations_after_time {
            let mut new_y : i64 = loc.0 + loc.2;
            let mut new_x : i64 = loc.1 + loc.3;

            new_y %= y_size;
            new_x %= x_size;

            if new_x < 0 {
                new_x += x_size;
            }

            if new_y < 0 {
                new_y += y_size;
            }

            loc.0 = new_y;
            loc.1 = new_x;
        }

        // Only start writing to file after 5500 iterations
        if time < 5500 {
            continue;
        }

        // Create grid as 2d collection
        let mut grid_raw : Vec<i64> = vec![0; grid_x_size * grid_y_size];
        let mut grid_base : Vec<_> = grid_raw.as_mut_slice().chunks_mut(grid_x_size).collect();
        let grid = grid_base.as_mut_slice();

        for x in &locations_after_time {
            let yyy : usize = x.0.try_into().expect("Failed to parse");
            let xxx : usize = x.1.try_into().expect("Failed to parse");
            grid[yyy][xxx] += 1;
        }

        // Commenting this out now - so I avoid writing to this file again
        // print_grid_to_file(grid, x_size.try_into().expect("Failed to parse"), y_size.try_into().expect("Failed to parse"), time);
    }

    return "7520".to_string();
}

#[allow(dead_code)]
fn print_grid_to_file(grid : &mut[&mut[i64]], x_size : usize, y_size : usize, time: i32) {

    let mut to_print : String = String::new();
    to_print.push_str(&format!("\nTime: {}", time));

    for y in 0..y_size {
        to_print.push_str("\n");
        for x in 0..x_size {

            if grid[y][x] == 0 {
                to_print.push_str("  ");
                continue;
            }

            to_print.push_str(&format!("{} ", grid[y][x]).to_string());
        }
    }

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&format!("output.txt").to_string())
        .unwrap();

    if let Err(e) = writeln!(file, "{}", to_print) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

/*

    SPOILER BELOW                   SPOILER BELOW                       SPOILER BELOW

    SPOILER BELOW                   SPOILER BELOW                       SPOILER BELOW

    SPOILER BELOW                   SPOILER BELOW                       SPOILER BELOW

    SPOILER BELOW                   SPOILER BELOW                       SPOILER BELOW

    SPOILER BELOW                   SPOILER BELOW                       SPOILER BELOW

    SPOILER BELOW                   SPOILER BELOW                       SPOILER BELOW

    SPOILER BELOW                   SPOILER BELOW                       SPOILER BELOW

    SPOILER BELOW                   SPOILER BELOW                       SPOILER BELOW

    SPOILER BELOW                   SPOILER BELOW                       SPOILER BELOW

    SPOILER BELOW                   SPOILER BELOW                       SPOILER BELOW

    SPOILER BELOW                   SPOILER BELOW                       SPOILER BELOW

    SPOILER BELOW                   SPOILER BELOW                       SPOILER BELOW

    SPOILER BELOW                   SPOILER BELOW                       SPOILER BELOW

    SPOILER BELOW                   SPOILER BELOW                       SPOILER BELOW

    SPOILER BELOW                   SPOILER BELOW                       SPOILER BELOW

    SPOILER BELOW                   SPOILER BELOW                       SPOILER BELOW

    SPOILER BELOW                   SPOILER BELOW                       SPOILER BELOW

    SPOILER BELOW                   SPOILER BELOW                       SPOILER BELOW

    SPOILER BELOW                   SPOILER BELOW                       SPOILER BELOW

    SPOILER BELOW                   SPOILER BELOW                       SPOILER BELOW

    SPOILER BELOW                   SPOILER BELOW                       SPOILER BELOW

    SPOILER BELOW                   SPOILER BELOW                       SPOILER BELOW

    SPOILER BELOW                   SPOILER BELOW                       SPOILER BELOW

    SPOILER BELOW                   SPOILER BELOW                       SPOILER BELOW

    SPOILER BELOW                   SPOILER BELOW                       SPOILER BELOW

    SPOILER BELOW                   SPOILER BELOW                       SPOILER BELOW

    SPOILER BELOW                   SPOILER BELOW                       SPOILER BELOW

    SPOILER BELOW                   SPOILER BELOW                       SPOILER BELOW

    SPOILER BELOW                   SPOILER BELOW                       SPOILER BELOW

    SPOILER BELOW                   SPOILER BELOW                       SPOILER BELOW

*/

/*

    CHRISTMAS TREE PICTURE!

    1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1
    1                                                           1
    1                                                           1
    1                                                           1
    1                                                           1
    1                             1                             1
    1                           1 1 1                           1
    1                         1 1 1 1 1                         1
    1                       1 1 1 1 1 1 1                       1
    1                     1 1 1 1 1 1 1 1 1                     1
    1                         1 1 1 1 1                         1
    1                       1 1 1 1 1 1 1                       1
    1                     1 1 1 1 1 1 1 1 1                     1
    1                   1 1 1 1 1 1 1 1 1 1 1                   1
    1                 1 1 1 1 1 1 1 1 1 1 1 1 1                 1
    1                     1 1 1 1 1 1 1 1 1                     1
    1                   1 1 1 1 1 1 1 1 1 1 1                   1
    1                 1 1 1 1 1 1 1 1 1 1 1 1 1                 1
    1               1 1 1 1 1 1 1 1 1 1 1 1 1 1 1               1
    1             1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1             1
    1                 1 1 1 1 1 1 1 1 1 1 1 1 1                 1
    1               1 1 1 1 1 1 1 1 1 1 1 1 1 1 1               1
    1             1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1             1
    1           1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1           1
    1         1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1         1
    1                           1 1 1                           1
    1                           1 1 1                           1
    1                           1 1 1                           1
    1                                                           1
    1                                                           1
    1                                                           1
    1                                                           1
    1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1

*/

