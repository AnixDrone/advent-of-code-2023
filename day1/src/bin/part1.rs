use std::collections::HashMap;
use std::fs;

fn main() {
    let content = match fs::read_to_string("src/input.txt") {
        Ok(content) => content,
        Err(_) => panic!("Failed to read file"),
    };

    //let words = content.split("\n").collect::<Vec<&str>>();
    //let numbers = words
    //    .iter()
    //    .map(|word| extract_numbers_from_word(word))
    //    .map(|number| number.parse::<i32>().unwrap())
    //    .collect::<Vec<i32>>();
    //print!("Sum of numbers: {:?}", numbers.iter().sum::<i32>());
    let result = extract_numbers_from_inccorect_words("one2twoxqzthree");
    print!("Result: {:?}", result);
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

fn extract_numbers_from_inccorect_words(word: &str) -> String {
    let number_map = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
        ("zero", "0"),
    ]);
    let mut buffer: String = String::new();
    let mut result = String::new();
    for c in word.chars() {
        if c.is_numeric() {
            continue;
        } else {
            buffer.push(c);
            if number_map.contains_key(&buffer[..]) {
                result.push_str(number_map.get(&buffer[..]).unwrap());
                buffer.clear();
            }
        }
    }
    return result;
}
