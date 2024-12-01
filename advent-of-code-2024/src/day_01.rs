use std::fs;
use std::time::Instant;

// https://adventofcode.com/2024/day/1

const DAY_STRING : &str = "day_01";
const TEST_RUNS : u32 = 1000;

fn part_1() -> String
{
    let input = get_input(false);

    let mut left_numbers : Vec<i32>;
    let mut right_numbers : Vec<i32>;

    (left_numbers, right_numbers) = split_input_into_two_collections(input);

    left_numbers.sort();
    right_numbers.sort();

    let mut distance : i32 = 0;
    for i in 0..left_numbers.len() {
        distance += (right_numbers[i] - left_numbers[i]).abs();
    }

    return distance.to_string();
}

fn part_2() -> String
{
    let input = get_input(false);

    let left_numbers : Vec<i32>;
    let right_numbers : Vec<i32>;

    (left_numbers, right_numbers) = split_input_into_two_collections(input);

    let mut similarity : i32 = 0;
    for i in 0..left_numbers.len() {
        let count : i32 = right_numbers.iter().filter(|&n| *n == left_numbers[i]).count().try_into().unwrap();
        similarity += left_numbers[i] * count;
    }

    return similarity.to_string();
}

pub fn solve() {
    let mut before = Instant::now();

    let part_1_result = part_1();
    for _ in 1..TEST_RUNS {
        part_1();
    }
    let part_1_time_elapsed = before.elapsed() / TEST_RUNS;

    before = Instant::now();

    let part_2_result = part_2();
    for _ in 1..TEST_RUNS {
        part_2();
    }
    let part_2_time_elapsed = before.elapsed() / TEST_RUNS;

    println!("\n-- Day 01 --");
    println!(" Part 1: {:15} elapsed: {:.2?} (average of 1000 runs)", part_1_result, part_1_time_elapsed);
    println!(" Part 2: {:15} elapsed: {:.2?} (average of 1000 runs)", part_2_result, part_2_time_elapsed);
}

fn get_input(use_test_data : bool) -> Vec<String> {
    let file_path = match use_test_data {
        true => format!("test_input/{}.txt", DAY_STRING),
        false => format!("input/{}.txt", DAY_STRING)
    };

    let data : Vec<String> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    return data;
}

fn split_input_into_two_collections(input : Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let mut left_numbers : Vec<i32> = vec![];
    let mut right_numbers : Vec<i32> = vec![];

    for line in input {
        let parts : Vec<String> = line.trim().split_whitespace().map(String::from).collect();

        let left : i32 = parts[0].parse()
            .expect("Failed to parse");
        let right : i32 = parts[1].parse()
            .expect("Failed to parse");

        left_numbers.push(left);
        right_numbers.push(right);
    }

    return (left_numbers, right_numbers)
}
