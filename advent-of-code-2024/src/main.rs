mod utilities;
mod day_01;
mod day_02;

use utilities::run_solution;

fn main() {
    run_solution("01", &day_01::part_1, &day_01::part_2);
    run_solution("02", &day_02::part_1, &day_02::part_2);

    println!("");
}
