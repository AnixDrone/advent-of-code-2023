use std::fs;

fn main() {
    let content = match fs::read_to_string("src/input.txt") {
        Ok(content) => content,
        Err(_) => panic!("Failed to read file"),
    };

    let words = content.split("\n").collect::<Vec<&str>>();
    print!("Part 1 {:?}", &words);
    let numbers = words
        .iter()
        .map(|word| extract_numbers_from_word(word))
        .collect::<Vec<i32>>();
    print!("Words: {:?}", numbers);
}

fn extract_numbers_from_word(word: &str) -> i32 {
    let numbers = word.chars().filter(|c| c.is_numeric()).collect::<String>();
    match numbers.parse() {
        Ok(n) => (numbers.chars().next().unwrap().to_string()
            + numbers.chars().last().unwrap().to_string())
        .parse()
        .unwrap(),
        Err(_) => 0,
    }
}
