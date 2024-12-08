use crate::utilities::get_input_lines;

// https://adventofcode.com/2024/day/2

const DAY_STRING : &str = "day_02";
const USE_TEST_DATA : bool = false;

#[allow(dead_code)]
pub fn part_1() -> String
{
    let input = get_input_lines(DAY_STRING, USE_TEST_DATA);
    let mut correct_counter : i32 = 0;

    for line in input {
        let levels : Vec<i32> = line.split_whitespace().map(|x|->i32{x.parse().unwrap()}).collect();

        let first : i32 = levels[0];
        let second : i32 = levels[1];

        if (first - second).abs() < 1 || (first - second).abs() > 3 {
            continue;
        }

        let increasing : bool = first < second;
        let mut prev = second;
        let mut correct : bool = true;

        if increasing {
            for &l in &levels[2..] {
                if l <= prev || (prev - l).abs() < 1 || (prev - l).abs() > 3 {
                    correct = false;
                    break;
                }
                prev = l;
            }
        } else {
            for &l in &levels[2..] {
                if l >= prev || (prev - l).abs() < 1 || (prev - l).abs() > 3 {
                    correct = false;
                    break;
                }
                prev = l;
            }
        }

        if correct {
            correct_counter += 1;
        }
    }

    return correct_counter.to_string();
}

#[allow(dead_code)]
pub fn part_2() -> String
{
    let input = get_input_lines(DAY_STRING, USE_TEST_DATA);
    let mut correct_counter : i32 = 0;

    for line in input {
        let levels : Vec<i32> = line.split_whitespace().map(|x|->i32{x.parse().unwrap()}).collect();

        for i in 0..levels.len() {
            let mut new_levels = levels.clone();
            new_levels.remove(i);

            if check_safety(&new_levels) && check_increase_decrease(&new_levels) {
                correct_counter += 1;
                break;
            }
        }
    }

    return correct_counter.to_string();
}

fn check_safety(levels : &Vec<i32>) -> bool {
    for i in 0..levels.len() - 1 {
        if (levels[i] - levels[i + 1]).abs() < 1 || (levels[i] - levels[i + 1]).abs() > 3 {
            return false;
        }
    }
    return true;
}

fn check_increase_decrease(levels : &Vec<i32>) -> bool {
    let increasing : bool = levels[0] < levels[1];

    for i in 0..levels.len() - 1 {
        if increasing {
            if levels[i] > levels[i + 1] {
                return false;
            }
        }
        else {
            if levels[i] < levels[i + 1] {
                return false;
            }
        }
    }
    return true;
}
