use crate::utilities::get_input_lines;
use regex::Regex;

// https://adventofcode.com/2024/day/3

const DAY_STRING : &str = "day_03";
const USE_TEST_DATA : bool = false;

pub fn part_1() -> String
{
    let input : Vec<String>;

    if USE_TEST_DATA {
        input = vec!["xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string()];
    } else {
        input = get_input_lines(DAY_STRING, USE_TEST_DATA);
    }

    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let mut sum : i32 = 0;

    for line in &input {
        for capture in re.captures_iter(line) {
            let mut numbers : String = capture[0].chars().skip(4).collect(); // remove "mul("
            numbers.pop(); // remove ")"
            let ab : Vec<i32> = numbers.split(',').map(|x| x.parse().expect("Failed to parse")).collect();
            sum += ab[0] * ab[1];
        }
    }

    return sum.to_string();
}

pub fn part_2() -> String
{
    let input : Vec<String>;

    if USE_TEST_DATA {
        input = vec!["xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string()];
    } else {
        input = get_input_lines(DAY_STRING, USE_TEST_DATA);
    }

    let re = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();
    let mut sum : i32 = 0;
    let mut enabled : bool = true;

    for line in &input {
        for capture in re.captures_iter(line) {

            if &capture[0] == "don't()"
            {
                enabled = false;
                continue;
            }

            if &capture[0] == "do()"
            {
                enabled = true;
                continue;
            }

            if enabled == false {
                continue;
            }

            let mut numbers : String = capture[0].chars().skip(4).collect(); // remove "mul("
            numbers.pop(); // remove ")"
            let ab : Vec<i32> = numbers.split(',').map(|x| x.parse().expect("Failed to parse")).collect();
            sum += ab[0] * ab[1];
        }
    }

    return sum.to_string();
}

