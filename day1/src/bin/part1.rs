use std::collections::HashMap;
use std::fs;

fn main() {
    let content = match fs::read_to_string("src/input.txt") {
        Ok(content) => content,
        Err(_) => panic!("Failed to read file"),
    };

    let words = content.split("\n").collect::<Vec<&str>>();
    let numbers = words
        .iter()
        .map(|word| extract_numbers_from_inccorect_words(word))
        .map(|number| number.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    print!("Sum of numbers: {:?}", numbers.iter().sum::<i32>());
}

fn fuzzy_match_word(word: &str, word1: &str) -> bool {
    for (i, c) in word.chars().enumerate() {
        if c != word1.chars().nth(i).unwrap() {
            return false;
        }
    }
    true
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
    if first_character == "0" {
        return second_character + "0";
    }
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
            buffer.clear();
            result.push(c);
            continue;
        } else {
            buffer.push(c);
            if number_map.contains_key(&buffer[..]) {
                result.push_str(number_map.get(&buffer[..]).unwrap());
                buffer = buffer.chars().last().unwrap().to_string();
            } else {
                let mut flag = false;
                for key in number_map.keys() {
                    if fuzzy_match_word(&buffer, &key) {
                        flag = true;
                    }
                }
                if !flag {
                    buffer.remove(0);
                }
            }
        }
    }

    let first_character = match result.chars().next() {
        Some(inner) => inner.to_string(),
        None => String::from("0"),
    };
    let second_character = match result.chars().last() {
        Some(inner) => inner.to_string(),
        None => String::from("0"),
    };
    return first_character + &second_character;
}
