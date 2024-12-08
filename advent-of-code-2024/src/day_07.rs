use std::collections::HashMap;

use crate::utilities::get_input_lines;

// https://adventofcode.com/2024/day/7

const DAY_STRING : &str = "day_07";
const USE_TEST_DATA : bool = false;

#[allow(dead_code)]
pub fn part_1() -> String
{
    return calculate_calibration_result(&get_input_lines(DAY_STRING, USE_TEST_DATA), false).to_string();
}

#[allow(dead_code)]
pub fn part_2() -> String
{
    return calculate_calibration_result(&get_input_lines(DAY_STRING, USE_TEST_DATA), true).to_string();
}

fn calculate_calibration_result(input : &Vec<String>, additional_operator : bool) -> i64 {
    let mut result : i64 = 0;
    let mut precomputed_perms : HashMap<usize, Vec<String>> = HashMap::new();

    for i in 1usize..=11usize {
        precomputed_perms.insert(i, get_permutations(additional_operator, "".to_string(), i.try_into().expect("Failed to parse")));
    }

    for line in input {
        let data : Vec<&str> = line.split_terminator(':').collect();
        let expected : i64 = data[0].parse().expect("Failed to parse");
        let numbers : Vec<i64> = data[1].split_whitespace().map(|x| x.parse().expect("Failed to parse")).collect();

        let sign_permutations = &precomputed_perms[&(numbers.len() - 1)];

        for perm in sign_permutations {
            let mut calculation : i64 = numbers[0];
            for i in 0..perm.len() {
                let operation : char = perm.chars().nth(i).unwrap();

                calculation = match operation {
                    '+' => calculation + numbers[i + 1],
                    '*' => calculation * numbers[i + 1],
                    '|' => format!("{}{}", calculation, numbers[i + 1]).parse().expect("Failed to parse"),
                    _ => 0
                };
            }
            if calculation == expected {
                result += expected;
                break;
            }
        }
    }

    return result;
}

// This could get optimized by using similar approach to caching previously calculated values when calculating large fibonacci numbers.
fn get_permutations(additional_operator : bool, perm: String, remaining : usize) -> Vec<String> {
    let simple_operators : Vec<char> = vec!['+', '*'];
    let advanceed_operators : Vec<char> = vec!['+', '*', '|'];
    let mut perms : Vec<String> = vec![];

    if remaining == 0 {
        perms.push(perm.clone());
    }
    else {
        for op in if additional_operator { advanceed_operators } else { simple_operators } {
            let new_perm = format!("{}{}", perm.clone(), op);
            perms.append(&mut get_permutations(additional_operator, new_perm, remaining - 1));
        }
    }

    return perms;
}
