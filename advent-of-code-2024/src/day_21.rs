use crate::utilities::get_input_lines;
use std::{collections::HashMap, usize};

// https://adventofcode.com/2024/day/21

const DAY_STRING : &str = "day_21";
const USE_TEST_DATA : bool = false;

fn press_directional_keyboard(current_arm_position : char, button_to_press : char) -> (char, String) {

    if current_arm_position == button_to_press {
        return (button_to_press, "".to_string());
    }

    let mut directional_keyboard_schemes : HashMap<(char, char), String> = HashMap::new();

    // current == A
    directional_keyboard_schemes.insert(('A', '^'), "<".to_string());
    directional_keyboard_schemes.insert(('A', '<'), "v<<".to_string());
    directional_keyboard_schemes.insert(('A', 'v'), "<v,v<".to_string());
    directional_keyboard_schemes.insert(('A', '>'), "v".to_string());

    // current == ^
    directional_keyboard_schemes.insert(('^', 'A'), ">".to_string());
    directional_keyboard_schemes.insert(('^', '<'), "v<".to_string());
    directional_keyboard_schemes.insert(('^', 'v'), "<".to_string());
    directional_keyboard_schemes.insert(('^', '>'), "v>".to_string());

    // current == <
    directional_keyboard_schemes.insert(('<', 'A'), ">>^".to_string());
    directional_keyboard_schemes.insert(('<', '^'), ">^".to_string());
    directional_keyboard_schemes.insert(('<', 'v'), ">".to_string());
    directional_keyboard_schemes.insert(('<', '>'), ">>".to_string());

    // current == v
    directional_keyboard_schemes.insert(('v', 'A'), ">^,^>".to_string());
    directional_keyboard_schemes.insert(('v', '^'), "^".to_string());
    directional_keyboard_schemes.insert(('v', '<'), "<".to_string());
    directional_keyboard_schemes.insert(('v', '>'), ">".to_string());

    // current == >
    directional_keyboard_schemes.insert(('>', 'A'), "^".to_string());
    directional_keyboard_schemes.insert(('>', '^'), "^<,<^".to_string());
    directional_keyboard_schemes.insert(('>', '<'), "<<".to_string());
    directional_keyboard_schemes.insert(('>', 'v'), "<".to_string());

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

fn translate_code_into_numeric_moves(code : String, cur_num_pos : char) -> (char, Vec<String>) {

    let mut cur_num_position: char = cur_num_pos;
    let mut required_moves : Vec<String> = vec!["".to_string()];

    for c in code.chars() {
        let (new_position, s) = press_numeric_keyboard(cur_num_position, c);

        let new_possible_moves : Vec<&str> = s.split_terminator(',').collect();

        // println!("Moved from {} to {}, path: {:?}", cur_num_position, new_position, new_possible_moves);

        cur_num_position = new_position;

        let mut new_required_moves : Vec<String> = vec![];

        for npm in &new_possible_moves {
            for rm in &required_moves {
                new_required_moves.push(format!("{}{}A", rm, npm));
            }
        }

        required_moves.clear();
        required_moves = std::mem::take(&mut new_required_moves);

        // println!("{:?}", required_moves);
    }

    return (cur_num_position, required_moves);
}

fn translate_num_moves_into_dir_moves(num_moves : Vec<String>, cur_dir_pos : char) -> (char, Vec<String>) {

    let mut cur_dir_position: char = cur_dir_pos;
    let mut all_required_dir_moves : Vec<String> = vec![];

    for num_moves in &num_moves {

        // let mut num_moves_with_a_click = "".to_string();

        // for nm in num_moves.chars() {
        //     num_moves_with_a_click.push_str(&format!("{}A", nm));
        // }

        let mut required_dir_moves : Vec<String> = vec!["".to_string()];

        for m in num_moves.chars() {
            let (new_position, s) = press_directional_keyboard(cur_dir_position, m);
            cur_dir_position = new_position;

            // if s == "" {
            //     continue;
            // }

            let new_possible_moves : Vec<&str> = s.split_terminator(',').collect();

            // println!("New possible moves for {}: {:?}", m, new_possible_moves);

            let mut new_required_moves : Vec<String> = vec![];

            if s == "" {
                for rm in &required_dir_moves {
                    new_required_moves.push(format!("{}A", rm));
                }
            } else {
                for npm in &new_possible_moves {
                    for rm in &required_dir_moves {
                        new_required_moves.push(format!("{}{}A", rm, npm));
                    }
                }
            }

            required_dir_moves.clear();
            required_dir_moves = std::mem::take(&mut new_required_moves);

            // println!("{:?}", required_dir_moves);
        }

        for x in required_dir_moves {
            all_required_dir_moves.push(x);
        }
    }

    // println!("Num -> Dir moves: '{:?}' -> '{:?}'", num_moves, all_required_dir_moves);

    return (cur_dir_position, all_required_dir_moves);
}

#[allow(dead_code)]
pub fn part_1() -> String
{
    let input = get_input_lines(DAY_STRING, USE_TEST_DATA);

    let mut result : usize = 0;

    for line in input {

        let cur_num_position: char = 'A';
        let cur_dir_position: char = 'A';

        // Phase I - translate code (ex: 029A) to moves on a numeric keypad
        let (_, required_num_moves) = translate_code_into_numeric_moves(line.clone(), cur_num_position);

        // Phase II - translate numeric keypad moves to directional keypad moves
        let mut required_moves = required_num_moves.clone();

        for i in 0..2 {
            println!("#{}: {}", i, required_moves.len());
            let (_, mut required_dir_moves) = translate_num_moves_into_dir_moves(required_moves.clone(), cur_dir_position);
            required_moves.clear();
            required_moves = std::mem::take(&mut required_dir_moves)
        }

        let mut shortest : usize = usize::MAX;

        for x in required_moves {
            if x.len() < shortest {
                shortest = x.len();
            }
        }

        let mut code_digits = line.clone();
        code_digits.pop().expect("Failed to pop").to_string();
        let code_as_num : usize = code_digits.parse().expect("Failed to parse");

        result += code_as_num * shortest;

        println!("Shortest combination length for code {}: {}. Digits: {}, value: {}", line, shortest, code_digits, code_as_num * shortest);
    }

    return result.to_string();
}

#[allow(dead_code)]
pub fn part_2() -> String
{
    let input = get_input_lines(DAY_STRING, USE_TEST_DATA);

    let mut result : usize = 0;

    for line in input {

        let cur_num_position: char = 'A';
        let cur_dir_position: char = 'A';

        // Phase I - translate code (ex: 029A) to moves on a numeric keypad
        let (_, required_num_moves) = translate_code_into_numeric_moves(line.clone(), cur_num_position);

        // Phase II - translate numeric keypad moves to directional keypad moves
        let mut required_moves = required_num_moves.clone();

        for i in 0..3 {
            println!("#{}: {}", i, required_moves.len());
            let (_, mut required_dir_moves) = translate_num_moves_into_dir_moves(required_moves.clone(), cur_dir_position);
            required_moves.clear();
            required_moves = std::mem::take(&mut required_dir_moves)
        }

        let mut shortest : usize = usize::MAX;

        for x in &required_moves {
            if x.len() < shortest {
                shortest = x.len();
            }
        }

        let mut code_digits = line.clone();
        code_digits.pop().expect("Failed to pop").to_string();
        let code_as_num : usize = code_digits.parse().expect("Failed to parse");

        result += code_as_num * shortest;

        println!("Shortest combination length for code {}: {}. Digits: {}, value: {}", line, shortest, code_digits, code_as_num * shortest);
    }

    return result.to_string();
}
