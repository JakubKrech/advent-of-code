use std::collections::HashMap;

use crate::utilities::get_input_lines;

// https://adventofcode.com/2024/day/23

const DAY_STRING : &str = "day_23";
const USE_TEST_DATA : bool = false;

#[allow(dead_code)]
pub fn part_1() -> String
{
    let input = get_input_lines(DAY_STRING, USE_TEST_DATA);

    let mut computer_connections : HashMap<&str, Vec<&str>> = HashMap::new();
    let mut sets_of_three : Vec<(String, String, String)> = vec![];

    for line in &input {
        let split_data : Vec<&str> = line.split_terminator('-').collect();
        computer_connections.entry(&split_data[0]).or_insert_with(Vec::new).push(split_data[1]);
        computer_connections.entry(&split_data[1]).or_insert_with(Vec::new).push(split_data[0]);
    }

    for comp_1 in &computer_connections {
        for connection in comp_1.1 {
            let comp_2 = &computer_connections[connection];

            for connection_2 in comp_2 {
                if connection_2 == comp_1.0 {
                    continue;
                }

                if comp_1.1.contains(connection_2) {
                    let mut three : Vec<String> = vec![comp_1.0.to_string(), connection.to_string(), connection_2.to_string()];
                    three.sort();
                    let tuple = (three[0].clone(), three[1].clone(), three[2].clone());
                    
                    if !sets_of_three.contains(&tuple) {
                        sets_of_three.push(tuple);
                    }
                }
            }
        }
    }

    let t_three_count = sets_of_three
        .iter()
        .filter(|x| x.0.starts_with('t') || x.1.starts_with('t') || x.2.starts_with('t'))
        .count();

    return t_three_count.to_string();
}

#[allow(dead_code)]
pub fn part_2() -> String
{
    let input = get_input_lines(DAY_STRING, USE_TEST_DATA);

    let mut computer_connections : HashMap<&str, Vec<&str>> = HashMap::new();

    for line in &input {
        let split_data : Vec<&str> = line.split_terminator('-').collect();
        let first = split_data[0];
        let second = split_data[1];

        computer_connections.entry(&first)
            .or_insert_with(Vec::new)
            .push(second);

        computer_connections.entry(&second)
            .or_insert_with(Vec::new)
            .push(first);
    }

    let mut all_combinations : Vec<Vec<&str>> = vec![vec![]];

    for computer in &computer_connections {

        let mut combinations : Vec<Vec<&str>> = vec![vec![computer.0]];
        let mut new_combinations : Vec<Vec<&str>> = vec![vec![]];

        for connection in computer.1 {

            for comb in &combinations {
                let mut can_be_added = true;
                let mut new_comb = comb.clone();

                for comb_computer in comb {
                    if !computer_connections[connection].contains(comb_computer) {
                        can_be_added = false;
                    }
                }

                if can_be_added {
                    new_comb.push(connection);
                }

                new_combinations.push(new_comb);
            }

            combinations.clear();
            combinations = std::mem::take(&mut new_combinations);
        }

        combinations.sort_by(|x, y| y.len().cmp(&x.len()));
        all_combinations.push(combinations[0].clone());
    }

    all_combinations.sort_by(|x, y| y.len().cmp(&x.len()));
    let mut longest_combination = all_combinations[0].clone();
    longest_combination.sort();

    return longest_combination.join(",");
}
