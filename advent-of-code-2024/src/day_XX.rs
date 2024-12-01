use std::fs;
use std::time::Instant;

// https://adventofcode.com/2024/day/XX

const DAY_STRING : &str = "XX";
const TEST_RUNS : u32 = 1;

fn part_1() -> String
{
    let input = get_input(false);

    return "".to_string();
}

fn part_2() -> String
{
    let input = get_input(false);

    return "".to_string();
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
    println!(" Part 1: {:15} elapsed: {:.2?} (average of 1000 runs)", part_1_result, part_1_time_elapsed);
    println!(" Part 2: {:15} elapsed: {:.2?} (average of 1000 runs)", part_2_result, part_2_time_elapsed);
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
