fn main() {
    //let text = include_str!("../input.txt");
    //let words = text.split("\n").collect::<Vec<&str>>();

    //print!("{:?}", words);
    let result = extract_numbers_from_word("");
    let number = match result.parse() {
        Ok(n) => n,
        Err(_) => 0,
    };
    println!("{}", number);
}

fn extract_numbers_from_word(word: &str) -> String {
    let numbers = word.chars().filter(|c| c.is_numeric()).collect::<String>();
    numbers
}
