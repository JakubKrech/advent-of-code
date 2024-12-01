use std::fs;
use std::time::Instant;

const TEST_RUNS : u32 = 1000;

pub fn run_solution(day: &str, part_1: &dyn Fn() -> String, part_2: &dyn Fn() -> String) {
    
    let part_1_result = part_1();
    
    let mut before = Instant::now();
    for _ in 0..TEST_RUNS {
        part_1();
    }
    let part_1_time_elapsed = before.elapsed() / TEST_RUNS;

    let part_2_result = part_2();

    before = Instant::now();
    for _ in 0..TEST_RUNS {
        part_2();
    }
    let part_2_time_elapsed = before.elapsed() / TEST_RUNS;

    println!("\n-- Day {} --", day);
    println!(" Part 1: {:15} elapsed: {:.2?} (average of 1000 runs)", part_1_result, part_1_time_elapsed);
    println!(" Part 2: {:15} elapsed: {:.2?} (average of 1000 runs)", part_2_result, part_2_time_elapsed);
}

pub fn get_input_lines(day_string : &str, use_test_data : bool) -> Vec<String> {
    let file_path = match use_test_data {
        true => format!("input_test/{}.txt", day_string),
        false => format!("input/{}.txt", day_string)
    };

    let data : Vec<String> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    return data;
}