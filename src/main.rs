mod calibration;
mod bag_game;
mod part_numbers;
mod scratch_cards;


fn main() {
    // let calibration = calibration::calculate("/home/luke/RustroverProjects/AdventOfCode/input/day_one.txt");
    // println!("calibration is {}", calibration);

    // let game_id_count = bag_game::calculate_possible("/home/luke/RustroverProjects/AdventOfCode/input/day_two.txt");
    // println!("game_id_count is {}", game_id_count);
    // let game_power = bag_game::calculate_power("/home/luke/RustroverProjects/AdventOfCode/input/day_two.txt");
    // println!("game power sum is {}", game_power);

    // let part_sum = part_numbers::get_gear_ratios("/home/luke/RustroverProjects/AdventOfCode/input/day_three.txt");
    // println!("Part number sum {}", part_sum);

    // let scratch_score = scratch_cards::calculate_scratch_score("/home/luke/RustroverProjects/AdventOfCode/input/day_four.txt");
    // println!("Scratch card score {}", scratch_score);

    let accumulated_scratch_score = scratch_cards::calculate_accumulating_score("/home/luke/RustroverProjects/AdventOfCode/input/day_four.txt");
    println!("Accumulating scratch score {}", accumulated_scratch_score);
}
