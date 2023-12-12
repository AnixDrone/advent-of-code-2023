use std::{fs, u8};

fn main() {
    let content = fs::read_to_string("src/sample_input.txt").expect("File not found");
    let lines = content.split("\n").collect::<Vec<&str>>();
    println!("{:?}", lines)
}

fn process_card_line(card_line: &str) -> (u8, String) {
    let card_details = card_line.split(":").collect::<Vec<&str>>();
    let card_title = card_line[0];
}
