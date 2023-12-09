use std::cmp::max;
use std::fs;

#[derive(Debug)]
struct Round {
    blue: u8,
    red: u8,
    green: u8,
}

impl Round {
    fn new(blue: u8, red: u8, green: u8) -> Round {
        Round { blue, red, green }
    }

    fn create_round(round: &str) -> Round {
        let colors = round.split(",").collect::<Vec<&str>>();
        let mut blue = 0;
        let mut red = 0;
        let mut green = 0;
        for color in colors {
            if color == "" {
                continue;
            }
            let color = color.split(" ").collect::<Vec<&str>>();
            let color_name = color[2];
            let color_value = color[1];
            match color_name {
                "blue" => blue = color_value.parse::<u8>().unwrap(),
                "red" => red = color_value.parse::<u8>().unwrap(),
                "green" => green = color_value.parse::<u8>().unwrap(),
                _ => println!("Unknown color"),
            }
        }
        Round::new(blue, red, green)
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl Game {
    fn new(id: u32, rounds: Vec<Round>) -> Game {
        Game { id, rounds }
    }

    fn max_ball_sum(&self) -> Round {
        let mut blue = 0;
        let mut red = 0;
        let mut green = 0;
        for round in &self.rounds {
            blue += round.blue;
            red += round.red;
            green += round.green;
        }
        Round::new(blue, red, green)
    }

    fn min_ball_sum(&self) -> Round {
        let mut blue = 0;
        let mut red = 0;
        let mut green = 0;
        for round in &self.rounds {
            blue = max(round.blue, blue);
            red = max(round.red, red);
            green = max(round.green, green);
        }
        Round::new(blue, red, green)
    }

    fn is_round_possible(&self, round: &Round) -> bool {
        for game_round in &self.rounds {
            if game_round.blue > round.blue
                || game_round.red > round.red
                || game_round.green > round.green
            {
                return false;
            }
        }
        return true;
    }
}

fn main() {
    let contents =
        fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");
    let games = contents
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| process_game(line))
        .collect::<Vec<Game>>();
    for game in &games {
        println!("Game {}", game.id);
        println!("Game rounds: {:?}", game.rounds);
        println!("Max ball sum: {:?}", game.max_ball_sum());
    }
    let input_round = Round::create_round(" 12 red, 13 green, 14 blue");
    let valid_games = games
        .iter()
        .filter(|game| game.is_round_possible(&input_round))
        .collect::<Vec<&Game>>();
    for game in &valid_games {
        println!("Game: {:?}", game.id);
    }
    print!(
        "Sum of valid game ids: {}",
        valid_games.iter().fold(0u32, |acc, game| acc + game.id)
    );
}

fn process_game(game: &str) -> Game {
    let games = game.split(":").collect::<Vec<&str>>();
    let id = games[0].split(" ").collect::<Vec<&str>>()[1]
        .parse::<u32>()
        .unwrap();
    let rounds = games[1]
        .split(";")
        .collect::<Vec<&str>>()
        .iter()
        .map(|round| Round::create_round(round))
        .collect::<Vec<Round>>();

    Game::new(id, rounds)
}
