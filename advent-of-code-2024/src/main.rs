mod utilities;
mod day_01; mod day_02; mod day_03; mod day_04; mod day_05; mod day_06; mod day_07; mod day_08;

use utilities::run_solution;

fn main() {
    run_solution("01", &day_01::part_1, &day_01::part_2);
    run_solution("02", &day_02::part_1, &day_02::part_2);
    run_solution("03", &day_03::part_1, &day_03::part_2);
    run_solution("04", &day_04::part_1, &day_04::part_2);
    run_solution("05", &day_05::part_1, &day_05::part_2);
    run_solution("06", &day_06::part_1, &day_06::part_2);
    run_solution("07", &day_07::part_1, &day_07::part_2);
    run_solution("08", &day_08::part_1, &day_08::part_2);
}
