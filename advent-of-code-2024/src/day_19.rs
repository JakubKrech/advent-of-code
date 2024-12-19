use std::collections::HashMap;

use crate::utilities::get_input_lines;

// https://adventofcode.com/2024/day/19

const DAY_STRING : &str = "day_19";
const USE_TEST_DATA : bool = false;

fn try_match_pattern(pattern : String, towels : Vec<&str>) -> bool {

    let mut possible : bool = false;

    if pattern.len() == 0 {
        return true;
    }

    for t in &towels {
        if possible {
            break;
        }

        if pattern.starts_with(*t) {
            possible |= try_match_pattern(pattern[t.len()..].to_string(), towels.clone());
        }
    }

    return possible;
}

fn count_match_pattern(memory : &mut HashMap<String, usize>, pattern : String, towels : Vec<&str>) -> usize {

    if memory.contains_key(&pattern) {
        return memory[&pattern];
    }

    if pattern.len() == 0 {
        return 1;
    }

    let mut count: usize = 0;

    for t in &towels {

        if pattern.starts_with(*t) {
            count += count_match_pattern(memory, pattern[t.len()..].to_string(), towels.clone());
        }
    }

    memory.insert(pattern.clone(), count);

    return count;
}

#[allow(dead_code)]
pub fn part_1() -> String
{
    let input = get_input_lines(DAY_STRING, USE_TEST_DATA);
    let towels : Vec<&str> = input[0].split_terminator(',').map(|l| l.trim()).collect();
    let mut patterns : Vec<String> = vec![];

    for i in 2..input.len() {
        patterns.push(input[i].clone());
    }

    let mut possible_patterns = 0;

    for pat in &patterns {

        if try_match_pattern(pat.clone(), towels.clone()) {
            possible_patterns += 1;
        }
    }

    return possible_patterns.to_string();
}

#[allow(dead_code)]
pub fn part_2() -> String
{
    let input = get_input_lines(DAY_STRING, USE_TEST_DATA);
    let towels : Vec<&str> = input[0].split_terminator(',').map(|l| l.trim()).collect();
    let mut patterns : Vec<String> = vec![];

    for i in 2..input.len() {
        patterns.push(input[i].clone());
    }

    let mut possible_patterns = 0;
    let mut memory : HashMap<String, usize> = HashMap::new();

    for pat in &patterns {
        count_match_pattern(&mut memory, pat.clone(), towels.clone());
        possible_patterns += memory[pat];
    }

    return possible_patterns.to_string();
}
