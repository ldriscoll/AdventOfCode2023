mod calibration;
mod bag_game;


fn main() {
    // let calibration = calibration::calculate("/home/luke/RustroverProjects/AdventOfCode/input/day_one.txt");
    // println!("calibration is {}", calibration);

    // let game_id_count = bag_game::calculate_possible("/home/luke/RustroverProjects/AdventOfCode/input/day_two.txt");
    // println!("game_id_count is {}", game_id_count);
    let game_power = bag_game::calculate_power("/home/luke/RustroverProjects/AdventOfCode/input/day_two.txt");
    println!("game power sum is {}", game_power);
}
