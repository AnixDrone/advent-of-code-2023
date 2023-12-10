use std::{fs, usize};

fn main() {
    let schematic = fs::read_to_string("src/sample_input.txt").expect("Error reading file");
    let lines = schematic
        .split("\n")
        .filter(|l| !l.is_empty())
        .collect::<Vec<&str>>();
    let mut numbers = vec![];
    let mut coordinates: Vec<(usize, usize)> = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        let mut num_flag = false;
        let mut num_buffer: Vec<(usize, usize)> = vec![];
        for (j, c) in line.chars().enumerate() {
            if c.is_numeric() {
                num_buffer.push((i, j));
                num_flag = true;
                if j == line.len() - 1 {
                    if valid_part2(&num_buffer, &lines) {
                        let mut num = String::new();
                        for (s, x) in &num_buffer {
                            num.push(get_value(*s, *x, &lines));
                        }
                        coordinates.extend(num_buffer.iter());
                        numbers.push(num.parse::<usize>().unwrap());
                    }
                    num_buffer.clear();
                }
            }
            if !c.is_numeric() && num_flag {
                num_flag = false;
                if valid_part2(&num_buffer, &lines) {
                    let mut num = String::new();
                    for (s, x) in &num_buffer {
                        num.push(get_value(*s, *x, &lines));
                    }
                    coordinates.extend(num_buffer.iter());
                    numbers.push(num.parse::<usize>().unwrap());
                }
                num_buffer.clear();
            }
        }
    }
    println!("{:?}", numbers);
    println!("{:?}", numbers.iter().sum::<usize>());
    let mut new_lines: Vec<String> = Vec::new();
    for (i, line) in lines.into_iter().enumerate() {
        let mut new_line = line.to_string();
        for (ci, cj) in &coordinates {
            if i == *ci {
                new_line.remove(*cj);
                new_line.insert(*cj, 'X');
            }
        }
        new_lines.push(new_line);
    }
    println!("{}", new_lines.join("\n"));
}

fn get_value(i: usize, j: usize, lines: &Vec<&str>) -> char {
    let line = match lines.get(i) {
        Some(v) => v,
        None => "",
    };
    match line.chars().nth(j) {
        Some(v) => v,
        None => '.',
    }
}

fn valid_part2(coordinates: &Vec<(usize, usize)>, lines: &Vec<&str>) -> bool {
    let mut valid = false;
    for (i, j) in coordinates {
        if *j > 0 && get_value(*i, *j - 1, lines) == '*' {
            valid = true;
            break;
        }
        if get_value(*i, *j + 1, lines) == '*' {
            valid = true;
            break;
        }
        if *i > 0 && get_value(*i - 1, *j, lines) == '*' {
            valid = true;
            break;
        }
        if get_value(*i + 1, *j, lines) == '*' {
            valid = true;
            break;
        }
        if *i > 0 && *j > 0 && get_value(*i - 1, *j - 1, lines) == '*' {
            valid = true;
            break;
        }
        if get_value(*i + 1, *j + 1, lines) == '*' {
            valid = true;
            break;
        }
        if *j > 0 && get_value(*i + 1, *j - 1, lines) == '*' {
            valid = true;
            break;
        }
        if *i > 0 && get_value(*i - 1, *j + 1, lines) == '*' {
            valid = true;
            break;
        }
    }
    valid
}
fn valid_num(coordinates: &Vec<(usize, usize)>, lines: &Vec<&str>) -> bool {
    let mut valid = false;
    for (i, j) in coordinates {
        if *j > 0
            && get_value(*i, *j - 1, lines) != '.'
            && !get_value(*i, *j - 1, lines).is_numeric()
        {
            valid = true;
            break;
        }
        if get_value(*i, *j + 1, lines) != '.' && !get_value(*i, *j + 1, lines).is_numeric() {
            valid = true;
            break;
        }
        if *i > 0
            && get_value(*i - 1, *j, lines) != '.'
            && !get_value(*i - 1, *j, lines).is_numeric()
        {
            valid = true;
            break;
        }
        if get_value(*i + 1, *j, lines) != '.' && !get_value(*i + 1, *j, lines).is_numeric() {
            valid = true;
            break;
        }
        if *i > 0
            && *j > 0
            && get_value(*i - 1, *j - 1, lines) != '.'
            && !get_value(*i - 1, *j - 1, lines).is_numeric()
        {
            valid = true;
            break;
        }
        if get_value(*i + 1, *j + 1, lines) != '.' && !get_value(*i + 1, *j + 1, lines).is_numeric()
        {
            valid = true;
            break;
        }
        if *j > 0
            && get_value(*i + 1, *j - 1, lines) != '.'
            && !get_value(*i + 1, *j - 1, lines).is_numeric()
        {
            valid = true;
            break;
        }
        if *i > 0
            && get_value(*i - 1, *j + 1, lines) != '.'
            && !get_value(*i - 1, *j + 1, lines).is_numeric()
        {
            valid = true;
            break;
        }
    }
    valid
}
