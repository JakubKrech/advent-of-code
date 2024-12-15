use crate::utilities::get_input_lines;
// use crate::utilities::print_grid;

// https://adventofcode.com/2024/day/15

const DAY_STRING : &str = "day_15";
const USE_TEST_DATA : bool = false;

#[allow(dead_code)]
pub fn part_1() -> String
{
    let input = get_input_lines(DAY_STRING, USE_TEST_DATA);

    let x_size : usize = input[0].len();
    let mut y_size : usize = 0;
    let mut instructions : String = String::new();

    // Read instructions
    for line in &input {
        if line.starts_with("#") {
            y_size += 1;
            continue;
        }
        if line == "" { continue; }
        instructions += line;
    }

    //  println!("Grid size: y:{}, x: {}", y_size, x_size);
    //  println!("Full instruction: {}", instructions);

    // Create grid as 2d collection
    let mut grid_raw : Vec<char> = vec!['.'; x_size * y_size];
    let mut grid_base : Vec<_> = grid_raw.as_mut_slice().chunks_mut(x_size).collect();
    let grid = grid_base.as_mut_slice();

    let mut robot_x : usize = 0;
    let mut robot_y : usize = 0;

    // Fill the grid
    for y in 0..y_size {
        for x in 0..x_size {
            grid[y][x] = input[y].chars().nth(x).expect("Failed to fill the grid");

            if grid[y][x] == '@' {
                robot_y = y.try_into().expect("Failed to parse");
                robot_x = x.try_into().expect("Failed to parse");
            }
        }
    }

    //  println!("\nInitial GRID:");
    // print_grid(grid, y_size, x_size);

    for i in 0..instructions.len() {
        
        // println!("");
        match instructions.chars().nth(i).expect("msg") {
            '^' => {
                // println!("^");
                if try_move_simple(grid, robot_y, robot_x, -1, 0) == true {
                    robot_y -= 1;
                }
            },
            '>' => {
                // println!(">");
                if try_move_simple(grid, robot_y, robot_x, 0, 1) == true {
                    robot_x += 1;
                }
            },
            'v' => {
                // println!("v");
                if try_move_simple(grid, robot_y, robot_x, 1, 0) == true {
                    robot_y += 1;
                }
            },
            '<' => {
                // println!("<");
                if try_move_simple(grid, robot_y, robot_x, 0, -1) == true {
                    robot_x -= 1;
                }
            },
            _ => println!("Unexpected instruction")
        }

        // println!("\nGRID after step {} ({}):", i, instructions.chars().nth(i).expect("msg"));
        // print_grid(grid, y_size, x_size);
        // println!("Robot position: y: {}, x: {}", robot_y, robot_x);
    }

    // println!("\nFinal GRID:");
    // print_grid(grid, y_size, x_size);

    let mut sum_of_boxes_coordinates : u64 = 0;

    for y in 0..y_size {
        for x in 0..x_size {
            if grid[y][x] == 'O' {
                let boxes_coord_value : u64 = (100 * y + x).try_into().expect("msg");
                sum_of_boxes_coordinates += boxes_coord_value;
            }
        }
    }

    return sum_of_boxes_coordinates.to_string();
}

fn try_move_simple(grid : &mut [&mut [char]], y_moved : usize, x_moved : usize, dy : i32, dx : i32) -> bool {

    let y_moved_i32: i32 = y_moved.try_into().expect("msg");
    let x_moved_i32: i32 = x_moved.try_into().expect("msg");

    let moved_to_y_usize : usize = (y_moved_i32 + dy).try_into().expect("msg");
    let moved_to_x_usize : usize = (x_moved_i32 + dx).try_into().expect("msg");

    if grid[moved_to_y_usize][moved_to_x_usize] == '#' {
        // println!("Found the wall");
        return false;
    }
    if grid[moved_to_y_usize][moved_to_x_usize] == '.' {
        // println!("Area empty, moving there");
        grid[moved_to_y_usize][moved_to_x_usize] = grid[y_moved][x_moved];
        grid[y_moved][x_moved] = '.';
    }
    else if grid[moved_to_y_usize][moved_to_x_usize] == 'O' || grid[moved_to_y_usize][moved_to_x_usize] == '[' || grid[moved_to_y_usize][moved_to_x_usize] == ']' {
        // println!("Box detected, check if it can be moved");
        if try_move_simple(grid, moved_to_y_usize, moved_to_x_usize, dy, dx) == false {
            return false;
        }

        // println!("Box moved, moving myself");
        grid[moved_to_y_usize][moved_to_x_usize] = grid[y_moved][x_moved];
        grid[y_moved][x_moved] = '.';
    }

    // println!("Returning true");
    return true;
}

#[allow(dead_code)]
pub fn part_2() -> String
{
    let input = get_input_lines(DAY_STRING, USE_TEST_DATA);

    let x_size : usize = input[0].len() * 2;
    let mut y_size : usize = 0;
    let mut instructions : String = String::new();
    
    // Read instructions
    for line in &input {
        if line.starts_with("#") {
            y_size += 1;
            continue;
        }

        if line == "" {
            continue;
        }

        instructions += line;
    }

    // println!("Grid size: y:{}, x: {}", y_size, x_size);
    // println!("Full instruction: {}", instructions);

    // Create grid as 2d collection
    let mut grid_raw : Vec<char> = vec!['.'; x_size * y_size];
    let mut grid_base : Vec<_> = grid_raw.as_mut_slice().chunks_mut(x_size).collect();
    let grid = grid_base.as_mut_slice();

    let mut robot_x : usize = 0;
    let mut robot_y : usize = 0;

    // Fill the grid
    for y in 0..y_size {
        for x in 0..x_size / 2 {

            match input[y].chars().nth(x).expect("Failed to fill the grid") {
                '#' => {
                    grid[y][x * 2] = '#';
                    grid[y][x * 2 + 1] = '#';
                },
                'O' => {
                    grid[y][x * 2] = '[';
                    grid[y][x * 2 + 1] = ']';
                }
                '.' => {
                    grid[y][x * 2] = '.';
                    grid[y][x * 2 + 1] = '.';
                }
                '@' => {
                    grid[y][x * 2] = '@';
                    grid[y][x * 2 + 1] = '.';
                    robot_y = y.try_into().expect("Failed to parse");
                    robot_x = (x * 2).try_into().expect("Failed to parse");
                }
                _ => println!("Wrong grid input")
            }
        }
    }

    // println!("\nInitial GRID:");
    // print_grid(grid, y_size, x_size);
    // println!("Robot position: y: {}, x: {}", robot_y, robot_x);

    for i in 0..instructions.len() {
        
        // println!("");
        let mut data : (bool, Vec<(usize, usize, char)>) = (false, vec![]);
        let mut dy : i32 = 0;
        let mut dx : i32 = 0;

        match instructions.chars().nth(i).expect("msg") {
            '^' => {
                // println!("^");
                data = try_move_wide_boxes_updown(grid, robot_y, robot_x, -1, 0);
                dy = -1;
                dx = 0;
                if data.0 == true {
                    robot_y -= 1;

                    // println!("will need to move boxes: {:?}", data.1);
                }
            },
            '>' => {
                // println!(">"); // ITS HORIZONTAL MOVE SO CALLING PART 1 MOVE SHOULD BE OK
                if try_move_simple(grid, robot_y, robot_x, 0, 1) == true {
                    robot_x += 1;
                }
            },
            'v' => {
                // println!("v");
                data = try_move_wide_boxes_updown(grid, robot_y, robot_x, 1, 0);
                dy = 1;
                dx = 0;
                if data.0 == true {
                    robot_y += 1;

                    // println!("will need to move boxes: {:?}", data.1);
                }
            },
            '<' => {
                // println!("<"); // ITS HORIZONTAL MOVE SO CALLING PART 1 MOVE SHOULD BE OK
                if try_move_simple(grid, robot_y, robot_x, 0, -1) == true {
                    robot_x -= 1;
                }
            },
            _ => println!("Unexpected instruction")
        }

        // Erase
        if data.1.len() > 0 {
            for elem in &data.1 {
                // println!("Erasing {} from {},{}", elem.2, elem.0, elem.1);
                grid[elem.0][elem.1] = '.';
            }
        }

        // Write
        if data.1.len() > 0 {
            for elem in &data.1 {
                let yy: i32 = elem.0.try_into().expect("msg");
                let xx: i32 = elem.1.try_into().expect("msg");
                let yyy : usize = (yy + dy).try_into().expect("msg");
                let xxx : usize = (xx + dx).try_into().expect("msg");
                // println!("Writing {} to {},{}", elem.2, yyy, xxx);
                grid[yyy][xxx] = elem.2;
            }
        }

        // println!("\nGRID after step {} ({}):", i, instructions.chars().nth(i).expect("msg"));
        // print_grid(grid, y_size, x_size);
        // println!("Robot position: y: {}, x: {}", robot_y, robot_x);
    }

    // println!("\nFinal GRID:");
    // print_grid(grid, y_size, x_size);
    // println!("Robot position: y: {}, x: {}", robot_y, robot_x);

    let mut sum_of_boxes_coordinates : u64 = 0;

    for y in 0..y_size {
        for x in 0..x_size {
            if grid[y][x] == '[' {
                let boxes_coord_value : u64 = (100 * y + x).try_into().expect("msg");
                sum_of_boxes_coordinates += boxes_coord_value;
            }
        }
    }

    return sum_of_boxes_coordinates.to_string();
}

fn try_move_wide_boxes_updown(grid : &mut [&mut [char]], moved_object_y : usize, moved_object_x : usize, dy : i32, dx : i32) -> (bool, Vec<(usize, usize, char)>) {

    // println!("Try to move part 2: {} {}   {} {} --------------------", moved_object_y, moved_object_x, dy, dx);

    let moved_object_y_i32: i32 = moved_object_y.try_into().expect("msg");
    let moved_object_x_i32: i32 = moved_object_x.try_into().expect("msg");

    let moved_to_y_usize : usize = (moved_object_y_i32 + dy).try_into().expect("msg");
    let moved_to_x_usize : usize = (moved_object_x_i32 + dx).try_into().expect("msg");

    let mut boxes_to_move : Vec<(usize, usize, char)> = vec![];

    // println!("  Will attempt to move to position: {} {}", moved_to_y_usize, moved_to_x_usize);

    // If its a robot and space is empty - move and return true early
    if grid[moved_to_y_usize][moved_to_x_usize] == '.' {
        // println!("  Area empty, moving there");
        boxes_to_move.push((moved_object_y, moved_object_x, grid[moved_object_y][moved_object_x]));
        // println!("  >>> empty boxes_to_move.push {} {}   {}", moved_object_y, moved_object_x, grid[moved_object_y][moved_object_x]);
        return (true, boxes_to_move);
    }

    // If there is a wall - return early with false
    if grid[moved_to_y_usize][moved_to_x_usize] == '#' {
        // println!("  Found the wall");
        return (false, boxes_to_move);
    }

    // That should never happen
    if grid[moved_to_y_usize][moved_to_x_usize] != '[' && grid[moved_to_y_usize][moved_to_x_usize] != ']' {
        // println!("  WTF??? Found: {}", grid[moved_to_y_usize][moved_to_x_usize]);
    }

    // We found a box... '[' or ']'

    let right_side_of_box_found : bool = grid[moved_to_y_usize][moved_to_x_usize] == ']';
    if right_side_of_box_found {
        // println!("  Found right side of the box ], check if it can be moved");
    } else {
        // println!("  Found left side of the box [, check if it can be moved");
    }

    // We need to treat this box as one entity, check if both halves can be moved and move them together
    let left_box_y : i32;
    let left_box_x : i32;
    let right_box_y : i32;
    let right_box_x : i32;

    if right_side_of_box_found {
        left_box_y = moved_object_y.try_into().expect("msg");
        left_box_x = (moved_object_x - 1).try_into().expect("msg");
        right_box_y = moved_object_y.try_into().expect("msg");
        right_box_x = moved_object_x.try_into().expect("msg");
    } else { // left side of box found
        left_box_y = moved_object_y.try_into().expect("msg");
        left_box_x = moved_object_x.try_into().expect("msg");
        right_box_y = moved_object_y.try_into().expect("msg");
        right_box_x = (moved_object_x + 1).try_into().expect("msg");
    }

    let left_moved_to_y_usize : usize = (left_box_y + dy).try_into().expect("msg");
    let left_moved_to_x_usize : usize = (left_box_x + dx).try_into().expect("msg");
    let right_moved_to_y_usize : usize = (right_box_y + dy).try_into().expect("msg");
    let right_moved_to_x_usize : usize = (right_box_x + dx).try_into().expect("msg");

    // println!("<<<<< CHECKING LEFT HALF OF BOX");
    let mut can_left_be_moved = try_move_wide_boxes_updown(grid, left_moved_to_y_usize, left_moved_to_x_usize, dy, dx);
    // println!("<<<<< CHECKING RIGHT HALF OF BOX");
    let mut can_right_be_moved = try_move_wide_boxes_updown(grid, right_moved_to_y_usize, right_moved_to_x_usize, dy, dx);

    if can_left_be_moved.0 == false || can_right_be_moved.0 == false {
        // println!("  Box cannot be moved [ {} | {} ]", can_left_be_moved.0, can_right_be_moved.0);
        return (false, boxes_to_move);
    }

    boxes_to_move.push((moved_object_y, moved_object_x, grid[moved_object_y][moved_object_x]));
    // println!("  >>> boxes_to_move.push {} {}   {}", moved_object_y, moved_object_x, grid[moved_object_y][moved_object_x]);

    boxes_to_move.append(&mut can_left_be_moved.1);
    boxes_to_move.append(&mut can_right_be_moved.1);

    // println!("  Returning true");
    return (true, boxes_to_move);
}
