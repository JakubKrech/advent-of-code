use std::collections::HashMap;

use crate::utilities::get_input_lines;

// https://adventofcode.com/2024/day/5

const DAY_STRING : &str = "day_05";
const USE_TEST_DATA : bool = false;

#[allow(dead_code)]
pub fn part_1() -> String
{
    let input = get_input_lines(DAY_STRING, USE_TEST_DATA);

    let mut required_predecessors : HashMap<String, Vec<String>> = HashMap::new();
    let mut pages_to_print : Vec<String> = vec![];

    let mut result : i32 = 0;

    let mut all_rules_read :bool = false;

    for line in input {
        if line == "" {
            all_rules_read = true;
            continue;
        }
        
        if !all_rules_read {
            let numbers : Vec<&str> = line.split_terminator('|').collect();
            let read_this_before = numbers[0];
            let read_this_after = numbers[1];

            if !required_predecessors.contains_key(read_this_after) {
                required_predecessors.insert(read_this_after.to_string(), vec![]);
            } 

            required_predecessors.get_mut(read_this_after)
                .expect("Failed to push elem into HashMap of Vectors")
                .push(read_this_before.to_string());
        } else {
            pages_to_print.push(line.to_string());
        }
    }

    // Go over the pages to print and check all elements on the right of current page number
    for page_to_print in pages_to_print {
        let pages : Vec<&str> = page_to_print.split_terminator(',').collect();
        let mut rule_broken : bool = false;

        for i in 0..pages.len() {
            if required_predecessors.contains_key(pages[i]) {
                if pages[i+1..].iter().any(|item| required_predecessors[pages[i]].contains(&item.to_string())) {
                    rule_broken = true;
                    break;
                }
            }
        }

        if !rule_broken {
            result += pages[pages.len() / 2].parse::<i32>().expect("Failed to parse");
        }
    }

    return result.to_string();
}

#[allow(dead_code)]
pub fn part_2() -> String
{
    let input = get_input_lines(DAY_STRING, USE_TEST_DATA);

    let mut required_predecessors : HashMap<String, Vec<String>> = HashMap::new();
    let mut pages_to_print : Vec<String> = vec![];

    let mut result : i32 = 0;

    let mut all_rules_read : bool = false;

    for line in input {
        if line == "" {
            all_rules_read = true;
            continue;
        }
        
        if !all_rules_read {
            let numbers : Vec<&str> = line.split_terminator('|').collect();
            let read_this_before = numbers[0];
            let read_this_after = numbers[1];

            if !required_predecessors.contains_key(read_this_after) {
                required_predecessors.insert(read_this_after.to_string(), vec![]);
            } 

            required_predecessors.get_mut(read_this_after)
                .expect("Failed to push elem into HashMap of Vectors")
                .push(read_this_before.to_string());
        } else {
            pages_to_print.push(line.to_string());
        }
    }

    let mut incorrect_pages_to_print : Vec<String> = vec![];

    // Go over the pages to print and check all elements on the right of current page number
    for page_to_print in pages_to_print {
        let pages : Vec<&str> = page_to_print.split_terminator(',').collect();

        for i in 0..pages.len() {
            if required_predecessors.contains_key(pages[i]) {
                if pages[i+1..].iter().any(|item| required_predecessors[pages[i]].contains(&item.to_string())) {
                    incorrect_pages_to_print.push(page_to_print);
                    break;
                }
            }
        }
    }

    for iptp in incorrect_pages_to_print {
        let pages : Vec<&str> = iptp.split_terminator(',').collect();
        let mut ordered : Vec<String> = vec![];

        ordered.push(pages[0].to_string());

        for new_elem in pages.iter().skip(1) {
            let mut inserted : bool = false;
            for i in 0..ordered.len() {
                if !required_predecessors.contains_key(new_elem.to_owned()) {
                    ordered.insert(0, new_elem.to_string());
                    inserted = true;
                    break;
                } else {
                    let req_pred = &required_predecessors[new_elem.to_owned()];
                    if !req_pred.contains(&ordered[i]) {
                        ordered.insert(i, new_elem.to_string());
                        inserted = true;
                        break;
                    }
                }
            }

            if !inserted {
                ordered.push(new_elem.to_string());
            }
        }

        result += ordered[ordered.len() / 2].parse::<i32>().expect("Failed to parse");
    }

    return result.to_string();
}

