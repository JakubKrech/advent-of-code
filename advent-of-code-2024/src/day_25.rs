use crate::utilities::get_input_lines;

// https://adventofcode.com/2024/day/25

const DAY_STRING : &str = "day_25";
const USE_TEST_DATA : bool = false;

#[allow(dead_code)]
pub fn part_1() -> String
{
    let input = get_input_lines(DAY_STRING, USE_TEST_DATA);

    let mut locks : Vec<(usize, usize, usize, usize, usize)> = vec![];
    let mut keys : Vec<(usize, usize, usize, usize, usize)> = vec![];

    let mut heights : (usize, usize, usize, usize, usize) = (0, 0, 0, 0, 0);
    let mut schematic_counter = 0;

    for line in &input {
        if line == "" {

            if input[schematic_counter * 8].chars().nth(0).unwrap() == '#' {
                locks.push(heights);
            } else {
                keys.push(heights);
            }
            schematic_counter += 1;

            heights = (0, 0, 0, 0, 0);
            continue;
        }

        if line.chars().nth(0).unwrap() == '#' {
            heights.0 += 1;
        }

        if line.chars().nth(1).unwrap() == '#' {
            heights.1 += 1;
        }

        if line.chars().nth(2).unwrap() == '#' {
            heights.2 += 1;
        }

        if line.chars().nth(3).unwrap() == '#' {
            heights.3 += 1;
        }

        if line.chars().nth(4).unwrap() == '#' {
            heights.4 += 1;
        }
    }

    if input[schematic_counter * 8].chars().nth(0).unwrap() == '#' {
        locks.push(heights);
    } else {
        keys.push(heights);
    }

    // for lock in &locks {
    //     println!("Lock: {:?}", lock);
    // }

    // for key in &keys {
    //     println!("Key: {:?}", key);
    // }

    let total_height : usize = 7;
    let mut matching_combinations : usize = 0;

    for l in &locks {
        for k in &keys {
            if l.0 + k.0 <= total_height &&
                l.1 + k.1 <= total_height &&
                l.2 + k.2 <= total_height &&
                l.3 + k.3 <= total_height &&
                l.4 + k.4 <= total_height {
                    matching_combinations += 1;
                }
        }
    }

    return matching_combinations.to_string();
}

#[allow(dead_code)]
pub fn part_2() -> String
{
    return "MERRY CHRISTMAS".to_string();
}
