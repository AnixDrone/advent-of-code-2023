fn main() {
    println!("Hello, world!");
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
