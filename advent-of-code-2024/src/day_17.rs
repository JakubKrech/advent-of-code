use crate::utilities::get_input_lines;

// https://adventofcode.com/2024/day/17

const DAY_STRING : &str = "day_17";
const USE_TEST_DATA : bool = false;

fn perform_operation(instruction_code : char, operand : char) {
    match instruction_code {
        '0' => adv(operand),
        '1' => bxl(operand),
        '2' => bst(operand),
        '3' => jnx(operand),
        '4' => bxc(operand),
        '5' => out(operand),
        '6' => bdv(operand),
        '7' => cdv(operand),
        _ => {
            println!("THIS SHOULD NOT HAPPEN - INCORRECT INSTRUCTION CODE");
        }
    }
}

fn get_combo_operand_value(combo_operand : char) -> i64 {
    unsafe {
        let result = match combo_operand {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => REGISTER_A,
            '5' => REGISTER_B,
            '6' => REGISTER_C,
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
}

fn get_literal_operand_value(literal_operand : char) -> i64 {
    let result : i64 = literal_operand.to_string().parse().unwrap();
    return result;
}

// The adv instruction (opcode 0) performs division. The numerator is the value in the A register.
// The denominator is found by raising 2 to the power of the instruction's combo operand.
// (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.)
// The result of the division operation is truncated to an integer and then written to the A register.
fn adv(combo_operand : char) {
    let combo_operand_value = get_combo_operand_value(combo_operand);

    // Write result to A register
    unsafe {
        let denominator = 2_i64.pow(combo_operand_value as u32);

        let result : i64 = REGISTER_A / denominator;

        // println!("adv: {} / {} = {} -> saving in REGISTER_A", REGISTER_A, denominator, result);
        REGISTER_A = result;

        // Apply instruction pointer offset
        INSTRUCTION_POINTER += 2;
    }
}

// The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the instruction's
// literal operand, then stores the result in register B.
fn bxl(literal_operand : char) {
    let literal_operand_value = get_literal_operand_value(literal_operand);

    unsafe {
        let result : i64 = REGISTER_B ^ literal_operand_value;
        // println!("bxl: {} ^ {} = {} -> saving in REGISTER_B", REGISTER_B, literal_operand_value, result);
        REGISTER_B = result; 

        // Apply instruction pointer offset
        INSTRUCTION_POINTER += 2;
    }
}

// The bst instruction (opcode 2) calculates the value of its combo operand modulo 8 (thereby
// keeping only its lowest 3 bits), then writes that value to the B register.
fn bst(combo_operand : char) {
    let combo_operand_value = get_combo_operand_value(combo_operand);
    let result = combo_operand_value % 8;

    unsafe {
        // println!("bst: {} % 8 = {} -> saving in REGISTER_B", combo_operand_value, result);
        REGISTER_B = result;

        // Apply instruction pointer offset
        INSTRUCTION_POINTER += 2;
    }
}

// The jnz instruction (opcode 3) does nothing if the A register is 0.
// However, if the A register is not zero, it jumps by setting the instruction
// pointer to the value of its literal operand; if this instruction jumps,
// the instruction pointer is not increased by 2 after this instruction.
fn jnx(literal_operand : char) {    
    unsafe {
        if REGISTER_A != 0 {
            let literal_operand_value = get_literal_operand_value(literal_operand);
            INSTRUCTION_POINTER = literal_operand_value;
            // println!("jnx: setting INSTRUCTION_POINTER to {}", INSTRUCTION_POINTER);
        } else {
            // Apply instruction pointer offset
            INSTRUCTION_POINTER += 2;
            // println!("jnx: doing nothing");
        }
    }
}

// The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C, then stores
// the result in register B. (For legacy reasons, this instruction reads an operand but ignores it.)
fn bxc(_ignored_operand : char) {
    unsafe {
        let result = REGISTER_B ^ REGISTER_C;
        // println!("bst: {} ^ {} = {} -> saving in REGISTER_B", REGISTER_B, REGISTER_C, result);
        REGISTER_B = result;

        // Apply instruction pointer offset
        INSTRUCTION_POINTER += 2;
    }
}

// The out instruction (opcode 5) calculates the value of its combo operand modulo 8, then outputs that
// value. (If a program outputs multiple values, they are separated by commas.)
fn out(combo_operand : char) {
    let combo_operand_value = get_combo_operand_value(combo_operand);
    let result = combo_operand_value % 8;
    // println!("out: {} % 8 = {} -> OUTPUT", combo_operand_value, result);
    unsafe {
        OUTPUT.push(result);

        // Apply instruction pointer offset
        INSTRUCTION_POINTER += 2;
    }
}

// The bdv instruction (opcode 6) works exactly like the adv instruction except that the result is stored
// in the B register. (The numerator is still read from the A register.)
fn bdv(combo_operand : char) {
    let combo_operand_value = get_combo_operand_value(combo_operand);

    // Write result to A register
    unsafe {
        let denominator = 2_i64.pow(combo_operand_value as u32);

        let result : i64 = REGISTER_A / denominator;

        // println!("bdv: {} / {} = {} -> saving in REGISTER_B", REGISTER_B, denominator, result);
        REGISTER_B = result;

        // Apply instruction pointer offset
        INSTRUCTION_POINTER += 2;
    }
}

// The cdv instruction (opcode 7) works exactly like the adv instruction except that the result is stored
// in the C register. (The numerator is still read from the A register.)
fn cdv(combo_operand : char) {
    let combo_operand_value = get_combo_operand_value(combo_operand);

    // Write result to A register
    unsafe {
        let denominator = 2_i64.pow(combo_operand_value as u32);

        let result : i64 = REGISTER_A / denominator;

        // println!("cdv: {} / {} = {} -> saving in REGISTER_C", REGISTER_C, denominator, result);
        REGISTER_C = result;

        // Apply instruction pointer offset
        INSTRUCTION_POINTER += 2;
    }
}

static mut REGISTER_A : i64 = 0;
static mut REGISTER_B : i64 = 0;
static mut REGISTER_C : i64 = 0;

static mut INSTRUCTION_POINTER : i64 = 0;
static mut OUTPUT : Vec<i64> = vec![];

#[allow(dead_code)]
pub fn part_1() -> String
{
    let input = get_input_lines(DAY_STRING, USE_TEST_DATA);

    unsafe {
        REGISTER_A = input[0].split_whitespace().last().unwrap().to_string().parse().expect("fail");
        REGISTER_B = input[1].split_whitespace().last().unwrap().to_string().parse().expect("fail");
        REGISTER_C = input[2].split_whitespace().last().unwrap().to_string().parse().expect("fail");
    }

    let instructions_string : String = input[4].split_whitespace().last().unwrap().to_string();

    let instructions: Vec<char> = instructions_string.split(',')
                                  .filter_map(|s| s.chars().next()) // Take the first character of each part
                                  .collect();

    // JUST TESTING
    // unsafe {
    //     adv('2');
    //     println!("InstructionPointer: {}, Registers: A={}, B={}, C={}", INSTRUCTION_POINTER, REGISTER_A, REGISTER_B, REGISTER_C);

    //     bxl('2');
    //     println!("InstructionPointer: {}, Registers: A={}, B={}, C={}", INSTRUCTION_POINTER, REGISTER_A, REGISTER_B, REGISTER_C);

    //     bst('4');
    //     println!("InstructionPointer: {}, Registers: A={}, B={}, C={}", INSTRUCTION_POINTER, REGISTER_A, REGISTER_B, REGISTER_C);

    //     jnx('2');
    //     println!("InstructionPointer: {}, Registers: A={}, B={}, C={}", INSTRUCTION_POINTER, REGISTER_A, REGISTER_B, REGISTER_C);

    //     REGISTER_C = 5;
    //     bxc('0');
    //     println!("InstructionPointer: {}, Registers: A={}, B={}, C={}", INSTRUCTION_POINTER, REGISTER_A, REGISTER_B, REGISTER_C);

    //     out('4');
    //     out('5');
    //     out('6');
    //     println!("InstructionPointer: {}, Registers: A={}, B={}, C={}", INSTRUCTION_POINTER, REGISTER_A, REGISTER_B, REGISTER_C);
    //     println!("Output: {:?}", OUTPUT);

    //     bdv('2');
    //     println!("InstructionPointer: {}, Registers: A={}, B={}, C={}", INSTRUCTION_POINTER, REGISTER_A, REGISTER_B, REGISTER_C);

    //     cdv('2');
    //     println!("InstructionPointer: {}, Registers: A={}, B={}, C={}", INSTRUCTION_POINTER, REGISTER_A, REGISTER_B, REGISTER_C);
    // }

    let mut result : String;

    unsafe {
        while INSTRUCTION_POINTER < instructions.len() as i64 {
            perform_operation(instructions[INSTRUCTION_POINTER as usize], instructions[(INSTRUCTION_POINTER + 1) as usize]);
        }
        result = OUTPUT.iter().map( |&id| id.to_string() + ",").collect(); 
    }
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

    // while counter < 500000000000000000 {
    //     counter += 4194304;
    //     unsafe {
    //         INSTRUCTION_POINTER = 0;
    //         REGISTER_A = counter;
    //         REGISTER_B = 0;
    //         REGISTER_C = 0;
    //         OUTPUT.clear();

    //         let mut output_len = 0;
    //         while INSTRUCTION_POINTER < instructions.len() as i64 {
    //             perform_operation(instructions[INSTRUCTION_POINTER as usize], instructions[(INSTRUCTION_POINTER + 1) as usize]);
            
    //             if output_len < OUTPUT.len() {
    //                 if instructions_i64[output_len] != OUTPUT[output_len] {
    //                     break;
    //                 } else {
    //                     if OUTPUT.len() >= 14 {
    //                         println!("#{}: {:?}, diff: {}", counter, OUTPUT, counter - prev);
    //                         prev = counter;
    //                     }
    //                 }
    //                 output_len = OUTPUT.len();
    //             }
    //         }
    //     }
    // }

    return "265061364597659".to_string();
}
