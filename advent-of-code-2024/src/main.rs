mod utilities;
use utilities::run_solution;

mod day_01; mod day_02; mod day_03; mod day_04; mod day_05;
mod day_06; mod day_07; mod day_08; mod day_09; mod day_10;
mod day_11; mod day_12; mod day_13; mod day_14; mod day_15;
mod day_16; mod day_17; mod day_18; mod day_19; mod day_20;
mod day_21; mod day_22; mod day_23; mod day_24; mod day_25;

fn main() {
    run_solution("01", &day_01::part_1, &day_01::part_2);
    run_solution("02", &day_02::part_1, &day_02::part_2);
    run_solution("03", &day_03::part_1, &day_03::part_2);
    run_solution("04", &day_04::part_1, &day_04::part_2);
    run_solution("05", &day_05::part_1, &day_05::part_2);
 
    run_solution("06", &day_06::part_1, &day_06::part_2);
    run_solution("07", &day_07::part_1, &day_07::part_2);
    run_solution("08", &day_08::part_1, &day_08::part_2);
    run_solution("09", &day_09::part_1, &day_09::part_2);
    run_solution("10", &day_10::part_1, &day_10::part_2);
 
    run_solution("11", &day_11::part_1, &day_11::part_2);
    run_solution("12", &day_12::part_1, &day_12::part_2);
    run_solution("13", &day_13::part_1, &day_13::part_2);
    run_solution("14", &day_14::part_1, &day_14::part_2);
    run_solution("15", &day_15::part_1, &day_15::part_2);
    
    run_solution("16", &day_16::part_1, &day_16::part_2);
    run_solution("17", &day_17::part_1, &day_17::part_2);
    run_solution("18", &day_18::part_1, &day_18::part_2);
    run_solution("19", &day_19::part_1, &day_19::part_2);    
    run_solution("20", &day_20::part_1, &day_20::part_2);

    run_solution("21", &day_21::part_1, &day_21::part_2);
    run_solution("22", &day_22::part_1, &day_22::part_2);
    run_solution("23", &day_23::part_1, &day_23::part_2);

    // day 24 idea:
    // calculate how many bits match -> try find swap of 2 outputs that will make one more bit match
    // then find another swap of 2 that will fix another bit
    // repeat two more times -> 4 pairs of outputs will be found, maybe it will work
    run_solution("24", &day_24::part_1, &day_24::part_2); 
    
    
    run_solution("25", &day_25::part_1, &day_25::part_2);
}
