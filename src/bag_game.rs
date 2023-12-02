use std::cmp;
use std::fs::read_to_string;


struct Play {
    red: u32,
    green: u32,
    blue: u32
}
struct Game {
    id: u32,
    plays: Vec<Play>
}

pub(crate) fn calculate_possible(filename: &str) -> u32 {
    let mut total: u32 = 0;
    for line in read_to_string(filename).unwrap().lines() {
        let game = get_game(line);
        if game_possible(&game) {
            println!("Game {} is possible", game.id);
            total = total + game.id
        } else {
            println!("Game {} is impossible", game.id);
        }
    }
    println!("Total {}", total);
    return total;
}

pub(crate) fn calculate_power(filename: &str) -> u32 {
    let mut total: u32 = 0;
    for line in read_to_string(filename).unwrap().lines() {
        let game = get_game(line);
        let game_power = game_power(&game);
        println!("Game {} has power {}", &game.id, game_power);
        total += game_power;
    }
    println!("Total {}", total);
    return total;
}

fn game_power(game: &Game) -> u32 {
    let empty_balls = Play { red:0u32, green:0u32, blue:0u32};

    let result = game.plays.iter().fold(empty_balls, |acc, x|
        Play {
            red: cmp::max(acc.red, x.red),
            green: cmp::max(acc.green, x.green),
            blue: cmp::max(acc.blue, x.blue)
        });
    println!("Game {} has min number of balls red {}, green {}, blue {}", game.id, result.red, result.green, result.blue);

    return result.red * result.green * result.blue;
}

fn game_possible(game: &Game) -> bool {
    return game.plays.iter().all(|play| play_possible(play))
}

fn play_possible(play: &Play) -> bool {
    let max_red= 12u32;
    let max_green= 13u32;
    let max_blue = 14u32;

    return play.red <= max_red && play.green <= max_green && play.blue <= max_blue
}

fn get_game(line: &str) -> Game {
    let split_colon = line.split_once(":").unwrap();
    let id:u32 = split_colon.0.split_once(" ").unwrap().1.parse().unwrap();
    let plays:Vec<Play> = split_colon.1.split(";").map(|r| parse_play(r)).collect();

    // println!("received {} {} plays from '{}'", id, plays.len(), line);
    return Game{id, plays};
}

fn parse_play(play_str: &str) -> Play {
    let mut red:u32 = 0;
    let mut green:u32 = 0;
    let mut blue:u32 = 0;

    play_str.split(",").for_each(
        |cl| {
            let count_colour = cl.trim().split_once(" ").unwrap();
            let count:u32 = count_colour.0.parse().unwrap();
            let colour = count_colour.1;
            if colour.eq("red") {
                red = count;
            } else if colour.eq("green") {
                green = count;
            } else if colour.eq("blue") {
                blue = count;
            } else {
                println!("Error colour {} not recognised with count {}", colour, count);
            }
        }
    );
    // println!("Red: {}, Green: {}, Blue: {} from '{}'", red, green, blue, play_str);
    return Play{
        red,
        green,
        blue
    };
}
