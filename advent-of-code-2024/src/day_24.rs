use std::collections::{HashMap, VecDeque};

use crate::utilities::get_input_lines;

// https://adventofcode.com/2024/day/24

const DAY_STRING : &str = "day_24";
const USE_TEST_DATA : bool = false;

#[allow(dead_code)]
pub fn part_1() -> String
{
    let input = get_input_lines(DAY_STRING, USE_TEST_DATA);

    //                        input1 operat input2 output calculated?
    let mut operations : Vec<(String, String, String, String, bool)> = vec![];
    let mut calculated_inputs : HashMap<String, bool> = HashMap::new();

    let mut first_section_read = false;
    for line in &input {
        if line == "" {
            first_section_read = true;
            continue;
        }

        if !first_section_read {
            let data : Vec<&str> = line.split_terminator(':').collect();
            calculated_inputs.insert(data[0].to_string(), data[1].trim().to_string() == "1");
        } else {
            let data : Vec<&str> = line.split_whitespace().collect();
            operations.push((data[0].to_string(), data[1].to_string(), data[2].to_string(), data[4].to_string(), false));
        }
    }

    let bin_decimal : isize = calculate(&mut operations, &mut calculated_inputs);

    return bin_decimal.to_string();
}

#[allow(dead_code)]
pub fn part_2() -> String
{
    // Part 2 is too compute-heavy to be simply bruteforced. After failing to come up
    // with smart, code based solution I tried to sort the operations and print additional
    // information about inputs/outputs of the operations. This proved to be a valid approach.
    // There were 8 operations that stood out from the crowd - they did not follow the schemes
    // that other similar operations followed. Using that information I was able to identify pairs
    // of operations that need to be swapped.

    // Disabling for now to avoid printing all the data when running all solutions together.
    if false {
        let input = get_input_lines(DAY_STRING, USE_TEST_DATA);

        //                        input1 operat input2 output
        let mut operations : Vec<(String, String, String, String)> = vec![];

        let mut first_section_read = false;
        for line in &input {
            if line == "" {
                first_section_read = true;
                continue;
            }

            if first_section_read {
                let data : Vec<&str> = line.split_whitespace().collect();
                operations.push((data[0].to_string(), data[1].to_string(), data[2].to_string(), data[4].to_string()));
            }
        }

        // DEBUG PRINTING OPERATIONS BY TYPE

        let mut sorted = operations.clone();
        sorted.sort_by(|x, y| x.3.cmp(&y.3));

        for op in &sorted {
            if op.3.starts_with("z"){
                print!("{} {:3} {} -> {}", op.0, op.1, op.2, op.3);
                if !op.0.starts_with("x") && !op.0.starts_with("y") {
                    let result = operations.iter().find(|&&(_, _, _, ref output)| *output == op.0).unwrap();
                    print!("  [{}: {} {:3} {}]", result.3, result.0, result.1, result.2);

                    let result_2 = operations.iter().find(|&&(_, _, _, ref output)| *output == op.2).unwrap();
                    print!("  [{}: {} {:3} {}]", result_2.3, result_2.0, result_2.1, result_2.2);
                }
                println!("");
            }
        }
        println!("");

        sorted = operations.clone();

        // swap so x is always on the right, its easier that way
        for entry in &mut sorted {
            if entry.2.starts_with("x") {
                let tmp = entry.0.clone();
                entry.0 = entry.2.clone();
                entry.2 = tmp.clone();
            }
        }

        sorted.sort_by(|x, y| x.0.cmp(&y.0));

        for op in &sorted {
            if (op.0.starts_with("x") || op.2.starts_with("x")) && op.1 == "XOR" {
                print!("{} {} {} -> {}", op.0, op.1, op.2, op.3);
                let results: Vec<_> = operations.iter()
                    .filter(|&&(ref first, _, ref second, _)| *first == op.3 || *second == op.3)
                    .collect();
                for r in &results {
                    print!("  [{} {:3} {} -> {}]", r.0, r.1, r.2, r.3);
                }

                println!("");
            }
        }
        println!("");
        for op in &sorted {
            if (op.0.starts_with("x") || op.2.starts_with("x")) && op.1 == "AND" {
                print!("{} {} {} -> {}", op.0, op.1, op.2, op.3);
                let results: Vec<_> = operations.iter()
                    .filter(|&&(ref first, _, ref second, _)| *first == op.3 || *second == op.3)
                    .collect();
                for r in &results {
                    print!("  [{} {:3} {} -> {}]", r.0, r.1, r.2, r.3);
                }
                println!("");
            }
        }

        println!("");

        sorted = operations.clone();
        sorted.sort_by(|x, y| x.1.cmp(&y.1));

        for op in &sorted {
            if !op.3.starts_with("z") && !op.0.starts_with("x") && !op.0.starts_with("y") && !op.2.starts_with("x") && !op.2.starts_with("y") {
                print!("{} {:3} {} -> {}", op.0, op.1, op.2, op.3);
                let results: Vec<_> = operations.iter()
                    .filter(|&&(ref first, _, ref second, _)| *first == op.3 || *second == op.3)
                    .collect();
                for r in &results {
                    print!("  [{} {:3} {} -> {}]", r.0, r.1, r.2, r.3);
                }
                println!("");
            }
        }
    }

    // Below outputs were selected "by hand" based on the above outputs - some of
    // the outputs stood out from the crowd.
    let mut to_swap : Vec<&str> = vec!["grm","z32","jcb","ndw","z39","twr","ggn","z10"];
    to_swap.sort();
    let outputs_to_swap : String = to_swap.join(",");

    return outputs_to_swap;
}

pub fn calculate(operations : &mut Vec<(String, String, String, String, bool)>, calculated_inputs : &mut HashMap<String, bool>) -> isize {

    let mut operations_to_calculate = operations.len();

    let mut calculated_anything_last_run = true;
    while operations_to_calculate > 0 && calculated_anything_last_run == true {
        calculated_anything_last_run = false;

        for i in 0..operations.len() {
            if operations[i].4 == true {
                continue;
            }

            if !calculated_inputs.contains_key(&operations[i].0) || !calculated_inputs.contains_key(&operations[i].2) {
                continue;
            }

            let input_1 : bool = calculated_inputs[&operations[i].0];
            let input_2 : bool = calculated_inputs[&operations[i].2];

            let result : bool = match operations[i].1.as_str() {
                "AND" => input_1 == true && input_2 == true,
                "OR" => input_1 == true || input_2 == true,
                "XOR" => input_1 != input_2,
                _ => false
            };

            calculated_inputs.insert(operations[i].3.clone(), result);
            operations[i].4 = true;
            operations_to_calculate -= 1;
            calculated_anything_last_run = true;
        }
    }

    // if calculated_anything_last_run == false {
    //     println!("Failed to calculate - would be stuck in a forever loop.");
    // }

    let bin_decimal : isize = get_decimal(&calculated_inputs, "z");

    return bin_decimal;
}

fn get_decimal(calculated_inputs : &HashMap<String, bool>, starting_with : &str) -> isize {
    let mut its = calculated_inputs.iter().collect::<Vec<_>>();
    its.sort();

    let mut binary_number_deque : VecDeque<String> = VecDeque::new();

    for x in its {
        if x.0.starts_with(starting_with) {
            binary_number_deque.push_front((*x.1 as i32).to_string());
        }
    }

    let bin : Vec<String> = binary_number_deque.into();
    let bin_as_string = bin.join("");
    let bin_decimal = isize::from_str_radix(&bin_as_string, 2).unwrap();

    return bin_decimal;
}
