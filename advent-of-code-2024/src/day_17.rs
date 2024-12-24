use crate::utilities::get_input_lines;

// https://adventofcode.com/2024/day/17

const DAY_STRING : &str = "day_17";
const USE_TEST_DATA : bool = false;

struct State {
    register_a: i64,
    register_b: i64,
    register_c: i64,
    instruction_pointer : i64,
    output : Vec<i64>
}

fn perform_operation(instruction_code : char, operand : char, state : &mut State) {
    match instruction_code {
        '0' => adv(operand, state),
        '1' => bxl(operand, state),
        '2' => bst(operand, state),
        '3' => jnx(operand, state),
        '4' => bxc(operand, state),
        '5' => out(operand, state),
        '6' => bdv(operand, state),
        '7' => cdv(operand, state),
        _ => {
            println!("THIS SHOULD NOT HAPPEN - INCORRECT INSTRUCTION CODE");
        }
    }
}

fn get_combo_operand_value(combo_operand : char, state : &State) -> i64 {
    let result = match combo_operand {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => state.register_a,
        '5' => state.register_b,
        '6' => state.register_c,
        '7' => {
            println!("THIS SHOULD NOT HAPPEN - COMBO OPERAND 7 SHOULD NOT APPEAR IN VALID PROGRAMS");
            return 0;
        }
        _ => {
            println!("THIS SHOULD NOT HAPPEN - COMBO OPERAND IN DEFAULT CASE");
            return 0;
        }
    };

    return result;
}

fn get_literal_operand_value(literal_operand : char) -> i64 {
    let result : i64 = literal_operand.to_string().parse().unwrap();
    return result;
}

// The adv instruction (opcode 0) performs division. The numerator is the value in the A register.
// The denominator is found by raising 2 to the power of the instruction's combo operand.
// (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.)
// The result of the division operation is truncated to an integer and then written to the A register.
fn adv(combo_operand : char, state : &mut State) {
    let combo_operand_value = get_combo_operand_value(combo_operand, state);

    // Write result to A register
    let denominator = 2_i64.pow(combo_operand_value as u32);

    let result : i64 = state.register_a / denominator;

    // println!("adv: {} / {} = {} -> saving in REGISTER_A", REGISTER_A, denominator, result);
    state.register_a = result;

    // Apply instruction pointer offset
    state.instruction_pointer += 2;
}

// The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the instruction's
// literal operand, then stores the result in register B.
fn bxl(literal_operand : char, state : &mut State) {
    let literal_operand_value = get_literal_operand_value(literal_operand);

    let result : i64 = state.register_b ^ literal_operand_value;
    // println!("bxl: {} ^ {} = {} -> saving in REGISTER_B", REGISTER_B, literal_operand_value, result);
    state.register_b = result; 

    // Apply instruction pointer offset
    state.instruction_pointer += 2;
}

// The bst instruction (opcode 2) calculates the value of its combo operand modulo 8 (thereby
// keeping only its lowest 3 bits), then writes that value to the B register.
fn bst(combo_operand : char, state : &mut State) {
    let combo_operand_value = get_combo_operand_value(combo_operand, state);
    let result = combo_operand_value % 8;

    // println!("bst: {} % 8 = {} -> saving in REGISTER_B", combo_operand_value, result);
    state.register_b = result;

    // Apply instruction pointer offset
    state.instruction_pointer += 2;
}

// The jnz instruction (opcode 3) does nothing if the A register is 0.
// However, if the A register is not zero, it jumps by setting the instruction
// pointer to the value of its literal operand; if this instruction jumps,
// the instruction pointer is not increased by 2 after this instruction.
fn jnx(literal_operand : char, state : &mut State) {    
    if state.register_a != 0 {
        let literal_operand_value = get_literal_operand_value(literal_operand);
        state.instruction_pointer = literal_operand_value;
        // println!("jnx: setting INSTRUCTION_POINTER to {}", INSTRUCTION_POINTER);
    } else {
        // Apply instruction pointer offset
        state.instruction_pointer += 2;
        // println!("jnx: doing nothing");
    }
}

// The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C, then stores
// the result in register B. (For legacy reasons, this instruction reads an operand but ignores it.)
fn bxc(_ignored_operand : char, state : &mut State) {
    let result = state.register_b ^ state.register_c;
    // println!("bst: {} ^ {} = {} -> saving in REGISTER_B", REGISTER_B, REGISTER_C, result);
    state.register_b = result;

    // Apply instruction pointer offset
    state.instruction_pointer += 2;
}

// The out instruction (opcode 5) calculates the value of its combo operand modulo 8, then outputs that
// value. (If a program outputs multiple values, they are separated by commas.)
fn out(combo_operand : char, state : &mut State) {
    let combo_operand_value = get_combo_operand_value(combo_operand, state);
    let result = combo_operand_value % 8;
    // println!("out: {} % 8 = {} -> OUTPUT", combo_operand_value, result);
    state.output.push(result);

    // Apply instruction pointer offset
    state.instruction_pointer += 2;
}

// The bdv instruction (opcode 6) works exactly like the adv instruction except that the result is stored
// in the B register. (The numerator is still read from the A register.)
fn bdv(combo_operand : char, state : &mut State) {
    let combo_operand_value = get_combo_operand_value(combo_operand, state);

    // Write result to A register
    let denominator = 2_i64.pow(combo_operand_value as u32);

    let result : i64 = state.register_a / denominator;

    // println!("bdv: {} / {} = {} -> saving in REGISTER_B", REGISTER_B, denominator, result);
    state.register_b = result;

    // Apply instruction pointer offset
    state.instruction_pointer += 2;
}

// The cdv instruction (opcode 7) works exactly like the adv instruction except that the result is stored
// in the C register. (The numerator is still read from the A register.)
fn cdv(combo_operand : char, state : &mut State) {
    let combo_operand_value = get_combo_operand_value(combo_operand, state);

    // Write result to A register
    let denominator = 2_i64.pow(combo_operand_value as u32);

    let result : i64 = state.register_a / denominator;

    // println!("cdv: {} / {} = {} -> saving in REGISTER_C", REGISTER_C, denominator, result);
    state.register_c = result;

    // Apply instruction pointer offset
    state.instruction_pointer += 2;
}

#[allow(dead_code)]
pub fn part_1() -> String
{
    let input = get_input_lines(DAY_STRING, USE_TEST_DATA);

    let mut state = State {
        register_a: input[0].split_whitespace().last().unwrap().to_string().parse().expect("fail"),
        register_b: input[1].split_whitespace().last().unwrap().to_string().parse().expect("fail"),
        register_c: input[2].split_whitespace().last().unwrap().to_string().parse().expect("fail"),
        instruction_pointer: 0,
        output: vec![]
    };

    let instructions_string : String = input[4].split_whitespace().last().unwrap().to_string();

    let instructions: Vec<char> = instructions_string.split(',')
                                  .filter_map(|s| s.chars().next()) // Take the first character of each part
                                  .collect();

    let mut result : String;

    while state.instruction_pointer < instructions.len() as i64 {
        perform_operation(instructions[state.instruction_pointer as usize], instructions[(state.instruction_pointer + 1) as usize], &mut state);
    }
    result = state.output.iter().map( |&id| id.to_string() + ",").collect(); 
    result.pop();

    return result.to_string();
}

#[allow(dead_code)]
pub fn part_2() -> String
{
    // Solving this part consisted of continuously checking what are the differences between REGISTER_A values
    // which allowed to match more and more digits of result/instructions input. After observing and adjusting
    // starting value and step few times I was able to get the first occurence of input which caused the result
    // to be identical to instruction input.

    // Commenting out below code to avoid repeating the computation when running all solutions in main.rs

    // Output of below code:

    // #7032631133083: [2, 4, 1, 7, 7, 5, 1, 7, 4, 6, 0, 3, 5, 5], diff: 7032631133083
    // #20986963100571: [2, 4, 1, 7, 7, 5, 1, 7, 4, 6, 0, 3, 5, 5], diff: 13954331967488
    // #42217003221915: [2, 4, 1, 7, 7, 5, 1, 7, 4, 6, 0, 3, 5, 5], diff: 21230040121344
    // #56171335189403: [2, 4, 1, 7, 7, 5, 1, 7, 4, 6, 0, 3, 5, 5], diff: 13954331967488
    // #77401375310747: [2, 4, 1, 7, 7, 5, 1, 7, 4, 6, 0, 3, 5, 5], diff: 21230040121344
    // #91355707278235: [2, 4, 1, 7, 7, 5, 1, 7, 4, 6, 0, 3, 5, 5], diff: 13954331967488
    // #112585747399579: [2, 4, 1, 7, 7, 5, 1, 7, 4, 6, 0, 3, 5, 5], diff: 21230040121344
    // #124323876242331: [2, 4, 1, 7, 7, 5, 1, 7, 4, 6, 0, 3, 5, 5], diff: 11738128842752
    // #124341056111515: [2, 4, 1, 7, 7, 5, 1, 7, 4, 6, 0, 3, 5, 5], diff: 17179869184
    // #126540079367067: [2, 4, 1, 7, 7, 5, 1, 7, 4, 6, 0, 3, 5, 5], diff: 2199023255552
    // #147770119488411: [2, 4, 1, 7, 7, 5, 1, 7, 4, 6, 0, 3, 5, 5], diff: 21230040121344
    // #161724451455899: [2, 4, 1, 7, 7, 5, 1, 7, 4, 6, 0, 3, 5, 5], diff: 13954331967488
    // #182954491577243: [2, 4, 1, 7, 7, 5, 1, 7, 4, 6, 0, 3, 5, 5], diff: 21230040121344
    // #196908823544731: [2, 4, 1, 7, 7, 5, 1, 7, 4, 6, 0, 3, 5, 5], diff: 13954331967488
    // #218138863666075: [2, 4, 1, 7, 7, 5, 1, 7, 4, 6, 0, 3, 5, 5], diff: 21230040121344
    // #232093195633563: [2, 4, 1, 7, 7, 5, 1, 7, 4, 6, 0, 3, 5, 5], diff: 13954331967488
    // #253323235754907: [2, 4, 1, 7, 7, 5, 1, 7, 4, 6, 0, 3, 5, 5], diff: 21230040121344
    // #265061364597659: [2, 4, 1, 7, 7, 5, 1, 7, 4, 6, 0, 3, 5, 5], diff: 11738128842752
    // #265061364597659: [2, 4, 1, 7, 7, 5, 1, 7, 4, 6, 0, 3, 5, 5, 3], diff: 0
    // #265061364597659: [2, 4, 1, 7, 7, 5, 1, 7, 4, 6, 0, 3, 5, 5, 3, 0], diff: 0         <----- FIRST OCCURENCE
    // #265078544466843: [2, 4, 1, 7, 7, 5, 1, 7, 4, 6, 0, 3, 5, 5], diff: 17179869184
    // #265078544466843: [2, 4, 1, 7, 7, 5, 1, 7, 4, 6, 0, 3, 5, 5, 3], diff: 0
    // #265078544466843: [2, 4, 1, 7, 7, 5, 1, 7, 4, 6, 0, 3, 5, 5, 3, 0], diff: 0
    // #267277567722395: [2, 4, 1, 7, 7, 5, 1, 7, 4, 6, 0, 3, 5, 5], diff: 2199023255552
    // #267277567722395: [2, 4, 1, 7, 7, 5, 1, 7, 4, 6, 0, 3, 5, 5, 3], diff: 0
    // #267277567722395: [2, 4, 1, 7, 7, 5, 1, 7, 4, 6, 0, 3, 5, 5, 3, 0], diff: 0
    // #288507607843739: [2, 4, 1, 7, 7, 5, 1, 7, 4, 6, 0, 3, 5, 5], diff: 21230040121344
    // #302461939811227: [2, 4, 1, 7, 7, 5, 1, 7, 4, 6, 0, 3, 5, 5], diff: 13954331967488

    // let input = get_input_lines(DAY_STRING, USE_TEST_DATA);
    // let instructions_string : String = input[4].split_whitespace().last().unwrap().to_string();
    // let instructions: Vec<char> = instructions_string.split(',')
    //                               .filter_map(|s| s.chars().next()) // Take the first character of each part
    //                               .collect();
    // let instructions_i64 : Vec<i64> = instructions.iter()
    //     .filter_map(|&c| c.to_digit(10)) // Convert char to digit (base 10)
    //     .map(|d| d as i64) // Convert the u32 result to i64
    //     .collect();

    // let mut prev : i64 = 0;
    // let mut counter : i64 = 62587803;

    // let mut state = State {
    //     register_a: counter,
    //     register_b: 0,
    //     register_c: 0,
    //     instruction_pointer: 0,
    //     output: vec![]
    // };

    // while counter < 500000000000000000 {
    //     counter += 4194304;

    //     state.register_a = counter;
    //     state.register_b = 0;
    //     state.register_c = 0;
    //     state.instruction_pointer = 0;
    //     state.output.clear();

    //     let mut output_len = 0;
    //     while state.instruction_pointer < instructions.len() as i64 {
    //         perform_operation(instructions[state.instruction_pointer as usize], instructions[(state.instruction_pointer + 1) as usize], &mut state);
        
    //         if output_len < state.output.len() {
    //             if instructions_i64[output_len] != state.output[output_len] {
    //                 break;
    //             } else {
    //                 if state.output.len() >= 14 {
    //                     println!("#{}: {:?}, diff: {}", counter, state.output, counter - prev);
    //                     prev = counter;
    //                 }
    //             }
    //             output_len = state.output.len();
    //         }
    //     }
    // }

    return "265061364597659".to_string();
}
