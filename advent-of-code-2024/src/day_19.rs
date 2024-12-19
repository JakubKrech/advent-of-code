use crate::utilities::get_input_lines;

// https://adventofcode.com/2024/day/19

const DAY_STRING : &str = "day_19";
const USE_TEST_DATA : bool = false;

fn try_match_pattern(pattern : String, towels : Vec<&str>) -> bool {

    // println!("Pattern: {}", pattern);
    let mut possible : bool = false;

    if pattern.len() == 0 {
        // println!("Pattern: {} is possible!", pattern);
        return true;
    }

    for t in &towels {
        // println!("Checking towel: {}", t);
        if possible {
            break;
        }

        if pattern.starts_with(*t) {
            // println!("Pattern '{}' starts with '{}'", pattern, t);
            possible |= try_match_pattern(pattern[t.len()..].to_string(), towels.clone());
        }
    }

    return possible;
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

    println!("Towels: {:?}", towels);
    println!("Patterns: {:?}", patterns);

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

    //let towels : Vec<&str> = input[0].split_terminator(',').map(|l| l.trim()).collect();

    //let mut patterns : Vec<String> = vec![];

    // for i in 2..input.len() {
    //     patterns.push(input[i].clone());
    // }

    // println!("Towels: {:?}", towels);
    // println!("Patterns: {:?}", patterns);

    // let mut possible_patterns = 0;

    // for pat in &patterns {
    //     possible_patterns += count_match_pattern(pat.clone(), towels.clone());
    // }

    return input.len().to_string();
}
