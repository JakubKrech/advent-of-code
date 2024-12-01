use crate::utilities::get_input_lines;

// https://adventofcode.com/2024/day/1

const DAY_STRING : &str = "day_01";
const USE_TEST_DATA : bool = false;

pub fn part_1() -> String
{
    let input = get_input_lines(DAY_STRING, USE_TEST_DATA);
    let (mut left_numbers, mut right_numbers) = split_input_into_two_collections(input);

    left_numbers.sort();
    right_numbers.sort();

    let mut distance : i32 = 0;
    for i in 0..left_numbers.len() {
        distance += (right_numbers[i] - left_numbers[i]).abs();
    }

    return distance.to_string();
}

pub fn part_2() -> String
{
    let input = get_input_lines(DAY_STRING, USE_TEST_DATA);
    let (left_numbers, right_numbers) = split_input_into_two_collections(input);

    let mut similarity : i32 = 0;
    for i in 0..left_numbers.len() {
        let count : i32 = right_numbers.iter().filter(|&n| *n == left_numbers[i]).count().try_into().unwrap();
        similarity += left_numbers[i] * count;
    }

    return similarity.to_string();
}

fn split_input_into_two_collections(input : Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let mut left_numbers : Vec<i32> = vec![];
    let mut right_numbers : Vec<i32> = vec![];

    for line in input {
        let parts : Vec<String> = line.trim().split_whitespace().map(String::from).collect();

        let left : i32 = parts[0].parse()
            .expect("Failed to parse");
        let right : i32 = parts[1].parse()
            .expect("Failed to parse");

        left_numbers.push(left);
        right_numbers.push(right);
    }

    return (left_numbers, right_numbers)
}
