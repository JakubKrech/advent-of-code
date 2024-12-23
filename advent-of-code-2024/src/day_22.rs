use std::collections::{HashMap, VecDeque};

use crate::utilities::get_input_lines;

// https://adventofcode.com/2024/day/22

const DAY_STRING : &str = "day_22";
const USE_TEST_DATA : bool = false;

fn evolve_secret(mut secret : i64) -> i64 {
    
    // Step 1
    secret = mix(secret, secret * 64);
    secret = prune(secret);

    // Step 2
    secret = mix(secret, (secret.clone() as f64 / 32.0).floor() as i64);
    secret = prune(secret);

    // Step 3
    secret = mix(secret, secret * 2048);
    secret = prune(secret);

    return secret;
}

fn mix(secret : i64, value : i64) -> i64 {
    return value ^ secret;
}

fn prune(secret : i64) -> i64 {
    return secret % 16777216;
}

#[allow(dead_code)]
pub fn part_1() -> String
{
    let input = get_input_lines(DAY_STRING, USE_TEST_DATA);
    let mut result : i64 = 0;

    for line in &input {
        let mut secret : i64 = line.parse().expect("Failed to parse");

        for _ in 0..2000 {
            secret = evolve_secret(secret);
        }

        result += secret;
    }

    return result.to_string();
}

#[allow(dead_code)]
pub fn part_2() -> String
{
    let input = get_input_lines(DAY_STRING, USE_TEST_DATA);

    let mut global_combinations_and_prices : HashMap<(i64, i64, i64, i64), i64> = HashMap::new();

    for line in &input {

        let mut queue : VecDeque<i64> = VecDeque::new();
        let mut local_combinations_and_prices : HashMap<(i64, i64, i64, i64), i64> = HashMap::new();

        // First digit
        let mut secret : i64 = line.parse().expect("Failed to parse");
        queue.push_front(secret % 10);

        // Second digit
        secret = evolve_secret(secret);
        queue.push_front(secret % 10);

        // Third digit
        secret = evolve_secret(secret);
        queue.push_front(secret % 10);

        // Fourth digit
        secret = evolve_secret(secret);
        queue.push_front(secret % 10);

        for _ in 3..2000 {
            secret = evolve_secret(secret);
            let new_last_digit = secret % 10;

            let changes = (queue[2] - queue[3], queue[1] - queue[2], queue[0] - queue[1], new_last_digit - queue[0]);
            queue.pop_back();
            queue.push_front(new_last_digit);
            
            if !local_combinations_and_prices.contains_key(&changes) {
                local_combinations_and_prices.insert(changes, new_last_digit);
            }
        }

        for x in local_combinations_and_prices {
            *global_combinations_and_prices.entry(x.0).or_insert(0) += x.1;
        }
    }

    let mut most_bananas : i64 = 0;

    for x in global_combinations_and_prices {
        if x.1 > most_bananas {
            most_bananas = x.1;
        }
    }

    return most_bananas.to_string();
}
