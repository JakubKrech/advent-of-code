use std::{collections::HashMap, mem};

use crate::utilities::get_input_lines;

// https://adventofcode.com/2024/day/11

const DAY_STRING : &str = "day_11";
const USE_TEST_DATA : bool = false;

#[allow(dead_code)]
pub fn part_1() -> String
{
    let input = get_input_lines(DAY_STRING, USE_TEST_DATA);

    let result = calculate_stones_after_blinks(input, 5, 5);

    return result.to_string();
}

#[allow(dead_code)]
pub fn part_2() -> String
{
    let input = get_input_lines(DAY_STRING, USE_TEST_DATA);

    let result = calculate_stones_after_blinks(input, 25, 3);

    return result.to_string();
}

fn calculate_stones_after_blinks(input : Vec<String>, blink_rounds : i32, blinks_per_round : i32) -> u64 {
    
    let input_numbers : Vec<u64> = input[0].split_whitespace().map(|x| x.parse().expect("Failed to parse")).collect();
    let mut memory : HashMap<u64, Vec<u64>> = HashMap::new();

    let mut distinct_numbers_count_a : HashMap<u64, u64> = HashMap::new();
    let mut distinct_numbers_count_b : HashMap<u64, u64> = HashMap::new();

    for x in input_numbers {
        distinct_numbers_count_a.insert(x, 1);
    }

    for _ in 0..blink_rounds {
        for n in &distinct_numbers_count_a {
            let res: Vec<u64>;
            
            if memory.contains_key(&n.0) {
                res = memory[&n.0].to_owned();
            } else {
                res = calculate_for_number(*n.0, blinks_per_round);
                memory.insert(*n.0, res.clone());
            }
            
            for num in &res {
                *distinct_numbers_count_b.entry(*num).or_insert(0) += n.1;
            }
        }

        distinct_numbers_count_a.clear();
        distinct_numbers_count_a = mem::take(&mut distinct_numbers_count_b);
    }

    let mut sum :u64 = 0;
    for x in &distinct_numbers_count_a {
        sum += x.1;
    }

    return sum;
}

fn calculate_for_number(num : u64, blinks : i32) -> Vec<u64> {
    let mut nums_1 : Vec<u64> = vec![num];
    let mut nums_2 : Vec<u64> = vec![];

    for _ in 0..blinks {
        for num in nums_1 {
            if num == 0 {
                nums_2.push(1);
            } else if num.to_string().len() % 2 == 0 {
                nums_2.push(num.to_string()[0..num.to_string().len() / 2].parse().expect("Failed to parse"));
                nums_2.push(num.to_string()[num.to_string().len() / 2..].parse().expect("Failed to parse"));
            } else {
                nums_2.push(num * 2024);
            }
        }

        nums_1 = mem::take(&mut nums_2);
    }

    return nums_1;
}
