use std::fs;
use std::time::Instant;

// https://adventofcode.com/2024/day/2

const DAY_STRING : &str = "02";
const TEST_RUNS : u32 = 1;

fn part_1() -> String
{
    let input = get_input(false);

    return input.len().to_string();
}

fn part_2() -> String
{
    let input = get_input(false);

    return input.len().to_string();
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

    println!("\n-- Day {} --", DAY_STRING);
    println!(" Part 1: {:15} elapsed: {:.2?} (average of {} runs)", part_1_result, part_1_time_elapsed, TEST_RUNS);
    println!(" Part 2: {:15} elapsed: {:.2?} (average of {} runs)", part_2_result, part_2_time_elapsed, TEST_RUNS);
}

fn get_input(use_test_data : bool) -> Vec<String> {
    let file_path = match use_test_data {
        true => format!("test_input/day_{}.txt", DAY_STRING),
        false => format!("input/day_{}.txt", DAY_STRING)
    };

    let data : Vec<String> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    return data;
}
