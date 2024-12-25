use std::collections::{HashMap, VecDeque};

use crate::utilities::get_input_lines;

// https://adventofcode.com/2024/day/24

const DAY_STRING : &str = "day_24";
const USE_TEST_DATA : bool = false;

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
    // println!("deciphering {}: {}, {}", starting_with, bin_as_string, bin_decimal);

    return bin_decimal;
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

            // println!("Calculating operation: {:?}, input_1: {}, input_2: {}", operations[i], input_1, input_2);

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
    let input = get_input_lines(DAY_STRING, USE_TEST_DATA);

    // //                        input1 operat input2 output calculated?
    // let mut operations : Vec<(String, String, String, String, bool)> = vec![];
    // let mut calculated_inputs : HashMap<String, bool> = HashMap::new();

    // let mut first_section_read = false;
    // for line in &input {
    //     if line == "" {
    //         first_section_read = true;
    //         continue;
    //     }

    //     if !first_section_read {
    //         let data : Vec<&str> = line.split_terminator(':').collect();
    //         calculated_inputs.insert(data[0].to_string(), data[1].trim().to_string() == "1");
    //     } else {
    //         let data : Vec<&str> = line.split_whitespace().collect();
    //         operations.push((data[0].to_string(), data[1].to_string(), data[2].to_string(), data[4].to_string(), false));
    //     }
    // }

    // for op in &operations {
    //     println!("Operation: {} {} {} -> {}, calculated: {}", op.0, op.1, op.2, op.3, op.4);
    //     // calculated_inputs.insert(op.3.clone();
    // }

    // for x in &calculated_inputs {
    //     println!("Wire: {}: {}", x.0, x.1);
    // }

    // let first_number : isize = get_decimal(&calculated_inputs, "x");
    // let second_number : isize = get_decimal(&calculated_inputs, "y");
    // let target_number = first_number + second_number;
    // let target_number_binary : String = format!("{:b}", target_number);

    // println!("numbers: {}, {}, need to find: {}", first_number, second_number, target_number);
    // println!("Target number {} in binary: {}", target_number, target_number_binary);

    // for i in 0..operations.len() {
    //     for j in i + 1..operations.len() {
    //         println!("{} {}", i, j);
    //         for k in i + 1..operations.len() {
    //             for l in k + 1..operations.len() {

    //                 if i == j || i == k || i == l || j == k || j == l || k == l {
    //                     continue;
    //                 }

    //                 // println!("{} {} {} {}", i, j, k, l);
    //                 let mut operations_new : Vec<(String, String, String, String, bool)> = operations.clone();
    //                 let mut calculated_inputs_new : HashMap<String, bool> = calculated_inputs.clone();

    //                 // Swap the operation output targets - FIRST PAIR
    //                 let tmp = operations_new[i].3.clone();
    //                 operations_new[i].3 = operations_new[j].3.clone();
    //                 operations_new[j].3 = tmp;

    //                 // Swap the operation output targets - SECOND PAIR
    //                 let tmp = operations_new[k].3.clone();
    //                 operations_new[k].3 = operations_new[l].3.clone();
    //                 operations_new[l].3 = tmp;

    //                 let res = calculate(&mut operations_new, &mut calculated_inputs_new);

    //                 // println!("Target: {}, Result: {}", target_number, res);

    //                 if target_number == res {
    //                     println!("FOUND!!!");
    //                     println!("Target: {}, Result: {}", target_number, res);

    //                     let mut swapped : Vec<&str> = vec![];
    //                     swapped.push(&operations[i].3);
    //                     swapped.push(&operations[j].3);
    //                     swapped.push(&operations[k].3);
    //                     swapped.push(&operations[l].3);

    //                     swapped.sort();
    //                     println!("{:?}", swapped);

    //                     return swapped.join(",");
    //                 }
    //             }
    //         }
    //     }
    // }

    return input.len().to_string();
}
