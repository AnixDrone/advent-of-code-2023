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

    fn sum(&self) -> u8 {
        self.blue + self.red + self.green
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
            println!("{}: {}", &color_name, &color_value);
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
    rounds: Vec<Round>,
}

impl Game {
    fn new(rounds: Vec<Round>) -> Game {
        Game { rounds }
    }

    fn sum(&self) -> u8 {
        let mut sum = 0;
        for round in &self.rounds {
            sum += round.sum();
        }
        sum
    }

    fn ball_sum(&self) -> Round {
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
}

fn main() {
    let contents =
        fs::read_to_string("src/sample_input.txt").expect("Something went wrong reading the file");
    let games = contents
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| process_game(line))
        .collect::<Vec<Game>>();
    println!("{:?}", games);
    for game in games {
        println!("Sum: {:?}", game.ball_sum());
    }
}

fn process_game(game: &str) -> Game {
    let games = game.split(":").collect::<Vec<&str>>();

    let rounds = games[1]
        .split(";")
        .collect::<Vec<&str>>()
        .iter()
        .map(|round| Round::create_round(round))
        .collect::<Vec<Round>>();

    Game::new(rounds)
}
