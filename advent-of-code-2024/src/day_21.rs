use crate::utilities::get_input_lines;
use std::{collections::HashMap, u64, usize};

// https://adventofcode.com/2024/day/21

const DAY_STRING : &str = "day_21";
const USE_TEST_DATA : bool = false;

#[allow(dead_code)]
pub fn part_1() -> String {
    let lowest_sum_of_complexities : u64 = find_lowest_sum_of_complexities(&get_input_lines(DAY_STRING, USE_TEST_DATA), 2);
    return lowest_sum_of_complexities.to_string();
}

#[allow(dead_code)]
pub fn part_2() -> String {
    let lowest_sum_of_complexities : u64 = find_lowest_sum_of_complexities(&get_input_lines(DAY_STRING, USE_TEST_DATA), 25);
    return lowest_sum_of_complexities.to_string();
}

pub fn find_lowest_sum_of_complexities(input_codes : &Vec<String>, directional_keypad_robot_count : usize) -> u64 {
    let mut lowest_sum_of_complexities : u64 = u64::MAX;

    // 4 out of 5 directional keypad buttons have two possible robot paths to click some buttons.
    // All of available path options need to be tested to find optimal solution.
    for option_a in 0..2 {
        for option_b in 0..2 {
            for option_c in 0..2 {
                for option_d in 0..2 {
                    let score = get_sum_of_complexities(input_codes, option_a, option_b, option_c, option_d, directional_keypad_robot_count);
                    if lowest_sum_of_complexities > score {
                        lowest_sum_of_complexities = score;
                    }
                }
            }
        }
    }

    return lowest_sum_of_complexities
}

pub fn get_sum_of_complexities(input_codes : &Vec<String>, opt_a : usize, opt_b : usize, opt_c : usize, opt_d : usize, directional_keypad_robot_count : usize) -> u64
{
    let mut sum_of_complexities : u64 = 0;

    for line in input_codes {
        // Phase I - translate code (ex: 029A) to moves on a numeric keypad. There might be multiple combinations as there are many button-button paths.
        let (_, directional_keypad_moves) = translate_numeric_keypad_into_directional_keypad_moves(line.clone());

        // Phase II - translate numeric keypad moves to directional keypad moves
        let mut directional_keypad_robot_arm_movements : Vec<HashMap<String, usize>> = vec![];

        // Phase III - start using HashSet to count how many types of movement will need to be done by each robot to optimize further computation
        for rdm in &directional_keypad_moves {
            let mut robot_arm_movements_count : HashMap<String, usize> = HashMap::new();
            let complete_robot_arm_movements = separate_list_of_moves_to_complete_movements_vector(rdm.to_string());
            for m in complete_robot_arm_movements {
                *robot_arm_movements_count.entry(m).or_insert(0) += 1;
            }
            directional_keypad_robot_arm_movements.push(robot_arm_movements_count);
        }

        // Phase IV - each robot will need to make more moves, to operate the previous robot's arm. Process the robot chain.
        for _ in 0..directional_keypad_robot_count {

            let mut new_directional_keypad_robot_arm_movements : Vec<HashMap<String, usize>> = vec![];

            for dkram in &directional_keypad_robot_arm_movements {
                let processed = process_robot_to_robot(dkram.clone(), opt_a, opt_b, opt_c, opt_d);

                for p in &processed {
                    new_directional_keypad_robot_arm_movements.push(p.clone());
                }
            }

            directional_keypad_robot_arm_movements = std::mem::take(&mut new_directional_keypad_robot_arm_movements);
        }

        // Phase V - Find lowest number of keypad button clicks that human needs to make
        let mut lowest_keypad_button_clicks : usize = usize::MAX;
        for x in &directional_keypad_robot_arm_movements {
            let mut keypad_button_clicks : usize = 0;
            for entry in x {
                keypad_button_clicks += entry.0.len() * entry.1;
            }
            if lowest_keypad_button_clicks > keypad_button_clicks {
                lowest_keypad_button_clicks = keypad_button_clicks;
            }
        }

        // Phase VI - Translate codes to u64 numbers and calculate the complexity
        let mut code_digits = line.clone();
        code_digits.pop().expect("Failed to pop").to_string();
        let code_as_num : u64 = code_digits.parse().expect("Failed to parse");

        sum_of_complexities += code_as_num * lowest_keypad_button_clicks as u64;
    }

    return sum_of_complexities;
}

fn process_robot_to_robot(move_strings : HashMap<String, usize>, a : usize, b : usize, c : usize, d : usize) -> Vec<HashMap<String, usize>> {
    let mut required_moves : Vec<HashMap<String, usize>> = vec![];
    required_moves.push(HashMap::new());

    for move_string in &move_strings {
        let multiplier = move_string.1;
        let mut curr_position = 'A';

        for move_char in move_string.0.chars() {
            let req_moves = press_directional_keyboard(curr_position, move_char, a, b, c, d);
            curr_position = req_moves.0;

            for rm in &mut required_moves {
                *rm.entry(format!("{}A", req_moves.1.clone())).or_insert(0) += 1 * multiplier;
            }
        }
    }

    return required_moves;
}

fn press_directional_keyboard(current_arm_position : char, button_to_press : char, opt_a : usize, opt_b : usize, opt_c : usize, opt_d : usize) -> (char, String) {

    if current_arm_position == button_to_press {
        return (button_to_press, "".to_string());
    }

    let mut directional_keyboard_schemes : HashMap<(char, char), String> = HashMap::new();

    // Some button-button moves have more than one path that can be reached with optimal ammount of robot arm moves.
    // All need to be tested to find out which combination is most optimal.
    let mut optional_schemes_a : Vec<((char, char), String)> = vec![];
    let mut optional_schemes_up : Vec<((char, char), String)> = vec![];
    let mut optional_schemes_down : Vec<((char, char), String)> = vec![];
    let mut optional_schemes_right : Vec<((char, char), String)> = vec![];

    // current == A
    directional_keyboard_schemes.insert(('A', '^'), "<".to_string());
    directional_keyboard_schemes.insert(('A', '<'), "v<<".to_string());
    directional_keyboard_schemes.insert(('A', '>'), "v".to_string());
    optional_schemes_a.push((('A', 'v'), "<v".to_string()));
    optional_schemes_a.push((('A', 'v'), "v<".to_string()));

    // current == ^
    directional_keyboard_schemes.insert(('^', 'A'), ">".to_string());
    directional_keyboard_schemes.insert(('^', '<'), "v<".to_string());
    directional_keyboard_schemes.insert(('^', 'v'), "v".to_string());
    optional_schemes_up.push((('^', '>'), ">v".to_string()));
    optional_schemes_up.push((('^', '>'), "v>".to_string()));

    // current == <
    directional_keyboard_schemes.insert(('<', 'A'), ">>^".to_string());
    directional_keyboard_schemes.insert(('<', '^'), ">^".to_string());
    directional_keyboard_schemes.insert(('<', 'v'), ">".to_string());
    directional_keyboard_schemes.insert(('<', '>'), ">>".to_string());

    // current == v
    directional_keyboard_schemes.insert(('v', '^'), "^".to_string());
    directional_keyboard_schemes.insert(('v', '<'), "<".to_string());
    directional_keyboard_schemes.insert(('v', '>'), ">".to_string());
    optional_schemes_down.push((('v', 'A'), ">^".to_string()));
    optional_schemes_down.push((('v', 'A'), "^>".to_string()));

    // current == >
    directional_keyboard_schemes.insert(('>', 'A'), "^".to_string());
    directional_keyboard_schemes.insert(('>', '<'), "<<".to_string());
    directional_keyboard_schemes.insert(('>', 'v'), "<".to_string());
    optional_schemes_right.push((('>', '^'), "<^".to_string()));
    optional_schemes_right.push((('>', '^'), "^<".to_string()));

    let aaa = &optional_schemes_a[opt_a];
    directional_keyboard_schemes.insert(aaa.0, aaa.1.clone());

    let bbb = &optional_schemes_up[opt_b];
    directional_keyboard_schemes.insert(bbb.0, bbb.1.clone());

    let ccc = &optional_schemes_down[opt_c];
    directional_keyboard_schemes.insert(ccc.0, ccc.1.clone());

    let ddd = &optional_schemes_right[opt_d];
    directional_keyboard_schemes.insert(ddd.0, ddd.1.clone());

    return (button_to_press, directional_keyboard_schemes[&(current_arm_position, button_to_press)].clone());
}

fn press_numeric_keyboard(current_arm_position : char, button_to_press : char) -> (char, String) {

    if current_arm_position == button_to_press {
        return (button_to_press, "".to_string());
    }

    let mut numeric_keyboard_schemes : HashMap<(char, char), String> = HashMap::new();

    // current = 'A'
    numeric_keyboard_schemes.insert(('A', '0'), "<".to_string());
    numeric_keyboard_schemes.insert(('A', '1'), "^<<".to_string());
    numeric_keyboard_schemes.insert(('A', '2'), "^<,<^".to_string());
    numeric_keyboard_schemes.insert(('A', '3'), "^".to_string());
    numeric_keyboard_schemes.insert(('A', '4'), "^^<<".to_string());
    numeric_keyboard_schemes.insert(('A', '5'), "^^<,<^^".to_string());
    numeric_keyboard_schemes.insert(('A', '6'), "^^".to_string());
    numeric_keyboard_schemes.insert(('A', '7'), "^^^<<".to_string());
    numeric_keyboard_schemes.insert(('A', '8'), "^^^<,<^^^".to_string());
    numeric_keyboard_schemes.insert(('A', '9'), "^^^".to_string());

    // current = '0'
    numeric_keyboard_schemes.insert(('0', 'A'), ">".to_string());
    numeric_keyboard_schemes.insert(('0', '1'), "^<".to_string());
    numeric_keyboard_schemes.insert(('0', '2'), "^".to_string());
    numeric_keyboard_schemes.insert(('0', '3'), "^>,>^".to_string());
    numeric_keyboard_schemes.insert(('0', '4'), "^^<".to_string());
    numeric_keyboard_schemes.insert(('0', '5'), "^^".to_string());
    numeric_keyboard_schemes.insert(('0', '6'), "^^>,>^^".to_string());
    numeric_keyboard_schemes.insert(('0', '7'), "^^^<".to_string());
    numeric_keyboard_schemes.insert(('0', '8'), "^^^".to_string());
    numeric_keyboard_schemes.insert(('0', '9'), "^^^>,>^^^".to_string());

    // current = '1'
    numeric_keyboard_schemes.insert(('1', 'A'), ">>v".to_string());
    numeric_keyboard_schemes.insert(('1', '0'), ">v".to_string());
    numeric_keyboard_schemes.insert(('1', '2'), ">".to_string());
    numeric_keyboard_schemes.insert(('1', '3'), ">>".to_string());
    numeric_keyboard_schemes.insert(('1', '4'), "^".to_string());
    numeric_keyboard_schemes.insert(('1', '5'), "^>,>^".to_string());
    numeric_keyboard_schemes.insert(('1', '6'), "^>>,>>^".to_string());
    numeric_keyboard_schemes.insert(('1', '7'), "^^".to_string());
    numeric_keyboard_schemes.insert(('1', '8'), "^^>,>^^".to_string());
    numeric_keyboard_schemes.insert(('1', '9'), "^^>>,>>^^".to_string());

    // current = '2'
    numeric_keyboard_schemes.insert(('2', 'A'), ">v,v>".to_string());
    numeric_keyboard_schemes.insert(('2', '0'), "v".to_string());
    numeric_keyboard_schemes.insert(('2', '1'), "<".to_string());
    numeric_keyboard_schemes.insert(('2', '3'), ">".to_string());
    numeric_keyboard_schemes.insert(('2', '4'), "^<,<^".to_string());
    numeric_keyboard_schemes.insert(('2', '5'), "^".to_string());
    numeric_keyboard_schemes.insert(('2', '6'), "^>,>^".to_string());
    numeric_keyboard_schemes.insert(('2', '7'), "^^<,<^^".to_string());
    numeric_keyboard_schemes.insert(('2', '8'), "^^".to_string());
    numeric_keyboard_schemes.insert(('2', '9'), "^^>,>^^".to_string());

    // current = '3'
    numeric_keyboard_schemes.insert(('3', 'A'), "v".to_string());
    numeric_keyboard_schemes.insert(('3', '0'), "<v,v<".to_string());
    numeric_keyboard_schemes.insert(('3', '1'), "<<".to_string());
    numeric_keyboard_schemes.insert(('3', '2'), "<".to_string());
    numeric_keyboard_schemes.insert(('3', '4'), "<<^,^<<".to_string());
    numeric_keyboard_schemes.insert(('3', '5'), "<^,^<".to_string());
    numeric_keyboard_schemes.insert(('3', '6'), "^".to_string());
    numeric_keyboard_schemes.insert(('3', '7'), "^^<<,<<^^".to_string());
    numeric_keyboard_schemes.insert(('3', '8'), "^^<,<^^".to_string());
    numeric_keyboard_schemes.insert(('3', '9'), "^^".to_string());

    // current = '4'
    numeric_keyboard_schemes.insert(('4', 'A'), ">>vv".to_string());
    numeric_keyboard_schemes.insert(('4', '0'), ">vv".to_string());
    numeric_keyboard_schemes.insert(('4', '1'), "v".to_string());
    numeric_keyboard_schemes.insert(('4', '2'), ">v,v>".to_string());
    numeric_keyboard_schemes.insert(('4', '3'), ">>v,v>>".to_string());
    numeric_keyboard_schemes.insert(('4', '5'), ">".to_string());
    numeric_keyboard_schemes.insert(('4', '6'), ">>".to_string());
    numeric_keyboard_schemes.insert(('4', '7'), "^".to_string());
    numeric_keyboard_schemes.insert(('4', '8'), "^>,>^".to_string());
    numeric_keyboard_schemes.insert(('4', '9'), "^>>,>>^".to_string());

    // current = '5'
    numeric_keyboard_schemes.insert(('5', 'A'), ">vv,vv>".to_string());
    numeric_keyboard_schemes.insert(('5', '0'), "vv".to_string());
    numeric_keyboard_schemes.insert(('5', '1'), "<v,v<".to_string());
    numeric_keyboard_schemes.insert(('5', '2'), "v".to_string());
    numeric_keyboard_schemes.insert(('5', '3'), ">v,v>".to_string());
    numeric_keyboard_schemes.insert(('5', '4'), "<".to_string());
    numeric_keyboard_schemes.insert(('5', '6'), ">".to_string());
    numeric_keyboard_schemes.insert(('5', '7'), "^<,<^".to_string());
    numeric_keyboard_schemes.insert(('5', '8'), "^".to_string());
    numeric_keyboard_schemes.insert(('5', '9'), "^>,>^".to_string());

    // current = '6'
    numeric_keyboard_schemes.insert(('6', 'A'), "vv".to_string());
    numeric_keyboard_schemes.insert(('6', '0'), "<vv,vv<".to_string());
    numeric_keyboard_schemes.insert(('6', '1'), "<<v,v<<".to_string());
    numeric_keyboard_schemes.insert(('6', '2'), "<v,v<".to_string());
    numeric_keyboard_schemes.insert(('6', '3'), "v".to_string());
    numeric_keyboard_schemes.insert(('6', '4'), "<<".to_string());
    numeric_keyboard_schemes.insert(('6', '5'), "<".to_string());
    numeric_keyboard_schemes.insert(('6', '7'), "<<^,^<<".to_string());
    numeric_keyboard_schemes.insert(('6', '8'), "<^,^<".to_string());
    numeric_keyboard_schemes.insert(('6', '9'), "^".to_string());

    // current = '7'
    numeric_keyboard_schemes.insert(('7', 'A'), ">>vvv".to_string());
    numeric_keyboard_schemes.insert(('7', '0'), ">vvv".to_string());
    numeric_keyboard_schemes.insert(('7', '1'), "vv".to_string());
    numeric_keyboard_schemes.insert(('7', '2'), ">vv,vv>".to_string());
    numeric_keyboard_schemes.insert(('7', '3'), "vv>>,>>vv".to_string());
    numeric_keyboard_schemes.insert(('7', '4'), "v".to_string());
    numeric_keyboard_schemes.insert(('7', '5'), ">v,v>".to_string());
    numeric_keyboard_schemes.insert(('7', '6'), ">>v,v>>".to_string());
    numeric_keyboard_schemes.insert(('7', '8'), ">".to_string());
    numeric_keyboard_schemes.insert(('7', '9'), ">>".to_string());

    // current = '8'
    numeric_keyboard_schemes.insert(('8', 'A'), "vvv>,>vvv".to_string());
    numeric_keyboard_schemes.insert(('8', '0'), "vvv".to_string());
    numeric_keyboard_schemes.insert(('8', '1'), "vv<,<vv".to_string());
    numeric_keyboard_schemes.insert(('8', '2'), "vv".to_string());
    numeric_keyboard_schemes.insert(('8', '3'), "vv>,>vv".to_string());
    numeric_keyboard_schemes.insert(('8', '4'), "<v,v<".to_string());
    numeric_keyboard_schemes.insert(('8', '5'), "v".to_string());
    numeric_keyboard_schemes.insert(('8', '6'), "v>,>v".to_string());
    numeric_keyboard_schemes.insert(('8', '7'), "<".to_string());
    numeric_keyboard_schemes.insert(('8', '9'), ">".to_string());

    // current = '9'
    numeric_keyboard_schemes.insert(('9', 'A'), "vvv".to_string());
    numeric_keyboard_schemes.insert(('9', '0'), "vvv<,<vvv".to_string());
    numeric_keyboard_schemes.insert(('9', '1'), "<<vv,vv<<".to_string());
    numeric_keyboard_schemes.insert(('9', '2'), "<vv,vv<".to_string());
    numeric_keyboard_schemes.insert(('9', '3'), "vv".to_string());
    numeric_keyboard_schemes.insert(('9', '4'), "<<v,v<<".to_string());
    numeric_keyboard_schemes.insert(('9', '5'), "<v,v<".to_string());
    numeric_keyboard_schemes.insert(('9', '6'), "v".to_string());
    numeric_keyboard_schemes.insert(('9', '7'), "<<".to_string());
    numeric_keyboard_schemes.insert(('9', '8'), "<".to_string());

    return (button_to_press, numeric_keyboard_schemes[&(current_arm_position, button_to_press)].clone());
}

fn separate_list_of_moves_to_complete_movements_vector(moves_string : String) -> Vec<String> {
    
    let mut complete_robot_arm_movements : Vec<String> = vec![];
    let mut start_index = 0;
    
    for (end_index, ch) in moves_string.char_indices() {
        if ch == 'A' {
            // Collect the substring up to the 'A'
            complete_robot_arm_movements.push(moves_string[start_index..=end_index].to_string());
            start_index = end_index + 1;
        }
    }
    return complete_robot_arm_movements;
}

fn translate_numeric_keypad_into_directional_keypad_moves(code : String) -> (char, Vec<String>) {

    let mut cur_num_position: char = 'A';
    let mut required_moves : Vec<String> = vec!["".to_string()];

    for c in code.chars() {
        let (new_position, s) = press_numeric_keyboard(cur_num_position, c);
        cur_num_position = new_position;
        let mut new_possible_moves : Vec<&str> = s.split_terminator(',').collect();

        if s == "" {
            new_possible_moves.push("");
        }

        let mut new_required_moves : Vec<String> = vec![];

        for npm in &new_possible_moves {
            for rm in &required_moves {
                new_required_moves.push(format!("{}{}A", rm, npm));
            }
        }

        required_moves.clear();
        required_moves = std::mem::take(&mut new_required_moves);
    }

    return (cur_num_position, required_moves);
}
