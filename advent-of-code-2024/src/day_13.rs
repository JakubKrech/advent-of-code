use crate::utilities::get_input_lines;

// https://adventofcode.com/2024/day/13

const DAY_STRING : &str = "day_13";
const USE_TEST_DATA : bool = false;

fn get_total_tokens_required(input : Vec<String>, claw_error : f64) -> f64 {
    let mut total_tokens_required  = 0.0;
    let token_a_cost = 3.0;
    let token_b_cost = 1.0;

    for i in 0..(1 + input.len() / 4) {
        let a : Vec<&str> = input[i * 4].split_terminator(',').collect();
        let b : Vec<&str> = input[i * 4 + 1].split_terminator(',').collect();
        let c : Vec<&str> = input[i * 4 + 2].split_terminator(',').collect();

        let ax : f64 = a[0][12..].parse().expect("Failed to parse");
        let ay : f64 = a[1][3..].parse().expect("Failed to parse");

        let bx : f64 = b[0][12..].parse().expect("Failed to parse");
        let by : f64 = b[1][3..].parse().expect("Failed to parse");

        let mut target_x : f64 = c[0][9..].parse().expect("Failed to parse");
        let mut target_y : f64 = c[1][3..].parse().expect("Failed to parse");

        target_x += claw_error;
        target_y += claw_error;

        let a_presses = (target_x * by - bx * target_y) / (by * ax - bx * ay);

        if a_presses % 1.0 != 0.0 {
            continue;
        }

        let b_presses= (target_x - a_presses * ax) / bx;

        if b_presses % 1.0 != 0.0 {
            continue;
        }

        let result = a_presses * ax + b_presses * bx;

        if target_x != result {
            continue;
        }

        total_tokens_required += a_presses * token_a_cost + b_presses * token_b_cost;
    }

    return total_tokens_required;
}

#[allow(dead_code)]
pub fn part_1() -> String
{
    let input = get_input_lines(DAY_STRING, USE_TEST_DATA);

    return get_total_tokens_required(input, 0.0).to_string();
}

#[allow(dead_code)]
pub fn part_2() -> String
{
    let input = get_input_lines(DAY_STRING, USE_TEST_DATA);

    return get_total_tokens_required(input, 10000000000000.0).to_string();
}
