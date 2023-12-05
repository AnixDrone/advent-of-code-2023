use std::fs;

fn main() {
    let content = match fs::read_to_string("src/input.txt") {
        Ok(content) => content,
        Err(_) => panic!("Failed to read file"),
    };

    let words = content.split("\n").collect::<Vec<&str>>();
    let numbers = words
        .iter()
        .map(|word| extract_numbers_from_word(word))
        .map(|number| number.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    print!("Sum of numbers: {:?}", numbers.iter().sum::<i32>());
}

fn extract_numbers_from_word(word: &str) -> String {
    let n = word.chars().filter(|c| c.is_numeric()).collect::<String>();
    let first_character = match n.chars().next() {
        Some(inner) => inner.to_string(),
        None => String::from("0"),
    };
    let second_character = match n.chars().last() {
        Some(inner) => inner.to_string(),
        None => String::from("0"),
    };
    return first_character + &second_character;
}
