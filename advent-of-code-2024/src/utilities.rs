use std::fs;
use std::time::Instant;

const TEST_RUNS : u32 = 1; // Set to 1 during development

pub fn run_solution(day: &str, part_1: &dyn Fn() -> String, part_2: &dyn Fn() -> String) {
    
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

    println!("\n-- Day {} --", day);
    println!(" Part 1: {:20} elapsed: {:.2?} (average of {} runs)", part_1_result, part_1_time_elapsed, TEST_RUNS);
    println!(" Part 2: {:20} elapsed: {:.2?} (average of {} runs)", part_2_result, part_2_time_elapsed, TEST_RUNS);
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

#[allow(dead_code)]
pub fn print_grid(grid : &mut[&mut[char]], y_size : usize, x_size : usize) {
    for y in 0..y_size {
        for x in 0..x_size {
            print!("{}", grid[y][x]);
        }
        println!("");
    }
}
