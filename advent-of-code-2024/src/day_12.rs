use crate::utilities::get_input_lines;

// https://adventofcode.com/2024/day/12

const DAY_STRING : &str = "day_12";
const USE_TEST_DATA : bool = false;

fn traverse_and_get_blob_elements(grid : &mut[&mut[char]], y_size : usize, x_size : usize, y : usize, x: usize) -> Vec<(usize, usize)> {
    let mut blob_elements : Vec<(usize, usize)> = vec![(y, x)];

    let uppercase = grid[y][x].clone();
    let lowercase = grid[y][x].to_lowercase().collect::<Vec<_>>()[0];

    grid[y][x] = lowercase;

    // Upper neighbour
    if y > 0 && uppercase == grid[y - 1][x] {
        blob_elements.append(&mut traverse_and_get_blob_elements(grid, y_size, x_size, y - 1, x));
    }

    // Right neighbour
    if x + 1 < x_size && uppercase == grid[y][x + 1] {
        blob_elements.append(&mut traverse_and_get_blob_elements(grid, y_size, x_size, y, x + 1));
    }

    // Lower neighbour
    if y + 1 < y_size && uppercase == grid[y + 1][x] {
        blob_elements.append(&mut traverse_and_get_blob_elements(grid, y_size, x_size, y + 1, x));
    }

    // Left neighbour
    if x > 0 && uppercase == grid[y][x - 1] {
        blob_elements.append(&mut traverse_and_get_blob_elements(grid, y_size, x_size, y, x - 1));
    }

    return blob_elements;
}

fn traverse(grid : &mut[&mut[char]], y_size : usize, x_size : usize, y : usize, x: usize) -> (i32, i32) {
    let mut area : i32 = 1;
    let mut borders : i32 = 4;

    let uppercase = grid[y][x].clone();
    let lowercase = grid[y][x].to_lowercase().collect::<Vec<_>>()[0];

    // Upper neighbour
    if y > 0 && (lowercase == grid[y - 1][x] || uppercase == grid[y - 1][x]) {
        borders -= 1;
    }
    // Right neighbour
    if x + 1 < x_size && (lowercase == grid[y][x + 1] || uppercase == grid[y][x + 1]) {
        borders -= 1;
    }
    // Lower neighbour
    if y + 1 < y_size && (lowercase == grid[y + 1][x] || uppercase == grid[y + 1][x]) {
        borders -= 1;
    }
    // Left neighbour
    if x > 0 && (lowercase == grid[y][x - 1] || uppercase == grid[y][x - 1]) {
        borders -= 1;
    }

    grid[y][x] = lowercase;

    // Upper neighbour
    if y > 0 && uppercase == grid[y - 1][x] {
        let t = traverse(grid, y_size, x_size, y - 1, x);
        area += t.0;
        borders += t.1;
    }

    // Right neighbour
    if x + 1 < x_size && uppercase == grid[y][x + 1] {
        let t = traverse(grid, y_size, x_size, y, x + 1);
        area += t.0;
        borders += t.1;
    }

    // Lower neighbour
    if y + 1 < y_size && uppercase == grid[y + 1][x] {
        let t = traverse(grid, y_size, x_size, y + 1, x);
        area += t.0;
        borders += t.1;
    }

    // Left neighbour
    if x > 0 && uppercase == grid[y][x - 1] {
        let t = traverse(grid, y_size, x_size, y, x - 1);
        area += t.0;
        borders += t.1;
    }

    return (area, borders);
}

#[allow(dead_code)]
pub fn part_1() -> String
{
    let input = get_input_lines(DAY_STRING, USE_TEST_DATA);

    // Get the grid size
    let x_size : usize = input[0].len();
    let y_size : usize = input.len();

    // Create grid as 2d collection
    let mut grid_raw = vec!['-'; (y_size) * (x_size)];
    let mut grid_base : Vec<_> = grid_raw.as_mut_slice().chunks_mut(y_size).collect();
    let grid = grid_base.as_mut_slice();

    // Fill the grid, but leave outer padding
    for y in 0..y_size {
        let line : Vec<char> = input[y].chars().collect();
        for x in 0..x_size {
            grid[y][x] = line[x].to_string().parse().expect("Failed to parse.");
        }
    }

    let mut result = 0;

    for y in 0..y_size {
        for x in 0..x_size {
            if grid[y][x].is_lowercase() {
                continue;
            }

            let traverse_score = traverse(grid, y_size, x_size, y, x);
            result += traverse_score.0 * traverse_score.1;
        }
    }

    return result.to_string();
}

#[allow(dead_code)]
pub fn part_2() -> String
{
    let input = get_input_lines(DAY_STRING, USE_TEST_DATA);

    // Get the grid size
    let x_size : usize = input[0].len();
    let y_size : usize = input.len();

    // Create grid as 2d collection
    let mut grid_raw = vec!['-'; (y_size) * (x_size)];
    let mut grid_base : Vec<_> = grid_raw.as_mut_slice().chunks_mut(y_size).collect();
    let grid = grid_base.as_mut_slice();

    // Fill the grid
    for y in 0..y_size {
        let line : Vec<char> = input[y].chars().collect();
        for x in 0..x_size {
            grid[y][x] = line[x].to_string().parse().expect("Failed to parse.");
        }
    }

    let mut result = 0;

    for y in 0..y_size {
        for x in 0..x_size {
            if grid[y][x].is_lowercase() {
                continue;
            }

            let mut blob_elements = traverse_and_get_blob_elements(grid, y_size, x_size, y, x);

            // Account for padding
            for ii in 0..blob_elements.len() {
                blob_elements[ii] = (blob_elements[ii].0 + 1, blob_elements[ii].1 + 1);
            }

            // Create small grid with size that allows fitting selected figure
            let y1 = blob_elements.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
            let x1 = blob_elements.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;

            let mut small_grid_size = y1;
            if x1 > y1 {
                small_grid_size = x1;
            }
            small_grid_size += 2; // padding

            // Create grid as 2d collection
            let mut small_grid_raw = vec!['-'; small_grid_size * small_grid_size];
            let mut small_grid_base : Vec<_> = small_grid_raw.as_mut_slice().chunks_mut(small_grid_size).collect();
            let small_grid = small_grid_base.as_mut_slice();

            // Fill the grid
            for elem in &blob_elements {
                small_grid[elem.0][elem.1] = grid[y][x];
            }

            let mut total_walls_for_blob = 0;

            // Scan going from top to bottom
            for yy in 0..small_grid_size {
                
                let mut bordering_elems : Vec<(usize, usize)> = vec![];

                for el in &blob_elements {
                    if el.0 != yy {
                        continue;
                    }

                    if small_grid[el.0 - 1][el.1] == '-' {
                        bordering_elems.push((el.0, el.1));
                    }
                }

                if bordering_elems.len() == 0 {
                    continue;
                }

                bordering_elems.sort_by(|a,b| a.1.cmp(&b.1));

                for ii in 1..bordering_elems.len() {
                    if (bordering_elems[ii].1 - bordering_elems[ii - 1].1) == 1 {
                        continue;
                    }
                    total_walls_for_blob += 1;
                }

                total_walls_for_blob += 1;
            }

            // Scan going from bottom to top
            for yy in 0..small_grid_size {
                
                let mut bordering_elems : Vec<(usize, usize)> = vec![];

                for el in &blob_elements {
                    if el.0 != yy {
                        continue;
                    }

                    if small_grid[el.0 + 1][el.1] == '-' {
                        bordering_elems.push((el.0, el.1));
                    }
                }

                if bordering_elems.len() == 0 {
                    continue;
                }

                bordering_elems.sort_by(|a,b| a.1.cmp(&b.1));

                for ii in 1..bordering_elems.len() {
                    if (bordering_elems[ii].1 - bordering_elems[ii - 1].1) == 1 {
                        continue;
                    }
                    total_walls_for_blob += 1;
                }

                total_walls_for_blob += 1;
            }

            // Scan going from left to right
            for xx in 0..small_grid_size {
                
                let mut bordering_elems : Vec<(usize, usize)> = vec![];

                for el in &blob_elements {
                    if el.1 != xx {
                        continue;
                    }

                    if small_grid[el.0][el.1 - 1] == '-' {
                        bordering_elems.push((el.0, el.1));
                    }
                }

                if bordering_elems.len() == 0 {
                    continue;
                }

                bordering_elems.sort_by(|a,b| a.0.cmp(&b.0));

                for ii in 1..bordering_elems.len() {
                    if (bordering_elems[ii].0 - bordering_elems[ii - 1].0) == 1 {
                        continue;
                    }
                    total_walls_for_blob += 1;
                }

                total_walls_for_blob += 1;
            }

            // Scan going from right to left
            for xx in 0..small_grid_size {
                
                let mut bordering_elems : Vec<(usize, usize)> = vec![];

                for el in &blob_elements {
                    if el.1 != xx {
                        continue;
                    }

                    if small_grid[el.0][el.1 + 1] == '-' {
                        bordering_elems.push((el.0, el.1));
                    }
                }

                if bordering_elems.len() == 0 {
                    continue;
                }

                bordering_elems.sort_by(|a,b| a.0.cmp(&b.0));

                for ii in 1..bordering_elems.len() {
                    if (bordering_elems[ii].0 - bordering_elems[ii - 1].0) == 1 {
                        continue;
                    }
                    total_walls_for_blob += 1;
                }

                total_walls_for_blob += 1;
            }

            result += total_walls_for_blob * blob_elements.len();
        }
    }

    return result.to_string();
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
