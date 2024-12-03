mod utilities;
mod day_01; mod day_02; mod day_03; mod day_04;

use utilities::run_solution;

fn main() {
    run_solution("01", &day_01::part_1, &day_01::part_2);
    run_solution("02", &day_02::part_1, &day_02::part_2);
    run_solution("03", &day_03::part_1, &day_03::part_2);
    run_solution("04", &day_04::part_1, &day_04::part_2);
}
