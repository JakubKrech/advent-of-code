use crate::utilities::get_input_lines;
use std::collections::HashMap;

// https://adventofcode.com/2024/day/21

const DAY_STRING : &str = "day_21";
const USE_TEST_DATA : bool = true;

// struct Keyboard {
//     current_position : char,
//     press_button : fn(button : char) -> String
// }

fn press_numeric_keyboard(current_arm_position : char, button_to_press : char) -> (char, String) {

    if current_arm_position == button_to_press {
        return ('A', "".to_string());
    }

    let mut numeric_keyboard_schemes : HashMap<(char, char), String> = HashMap::new();

    // current = 'A'
    numeric_keyboard_schemes.insert(('A', '0'), "<".to_string());
    numeric_keyboard_schemes.insert(('A', '1'), "^<<,<<^".to_string());
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

#[allow(dead_code)]
pub fn part_1() -> String
{
    let input = get_input_lines(DAY_STRING, USE_TEST_DATA);

    for line in &input {

        let mut cur_position: char = 'A';
        let mut required_moves = "".to_owned();

        for c in line.chars() {
            let (new_position, s) = press_numeric_keyboard(cur_position, c);

            let first_move : String = s.split_terminator(',').collect();

            cur_position = new_position;
            required_moves.push_str(format!("{}{}", first_move, 'A').as_ref());
            println!("{}", required_moves);
        }

        println!("Moves for '{}': {}", line, required_moves);
    }

    return input.len().to_string();
}

#[allow(dead_code)]
pub fn part_2() -> String
{
    let input = get_input_lines(DAY_STRING, USE_TEST_DATA);

    return input.len().to_string();
}
