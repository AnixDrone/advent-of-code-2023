use std::{fs, usize};

fn main() {
    let schematic = fs::read_to_string("src/input.txt").expect("Error reading file");
    let lines = schematic
        .split("\n")
        .filter(|l| !l.is_empty())
        .collect::<Vec<&str>>();
    let mut numbers = vec![];
    let mut coordinates: Vec<Vec<(usize, usize)>> = Vec::new();
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
                        coordinates.push(num_buffer.clone());
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
                    coordinates.push(num_buffer.clone());
                    numbers.push(num.parse::<usize>().unwrap());
                }
                num_buffer.clear();
            }
        }
    }
    let mut star_coordinates: Vec<Vec<Vec<(usize, usize)>>> = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '*' {
                let mut buffer_cord = vec![];
                for cord_set in &coordinates {
                    let mut triggered = false;
                    for (ci, cj) in cord_set {
                        if *ci + 1 == i && *cj == j {
                            triggered = true;
                            break;
                        }
                        if *ci == i && *cj + 1 == j {
                            triggered = true;
                            break;
                        }
                        if *ci > 0 && *ci - 1 == i && *cj == j {
                            triggered = true;
                            break;
                        }
                        if *cj > 0 && *ci == i && *cj - 1 == j {
                            triggered = true;
                            break;
                        }
                        if *ci + 1 == i && *cj + 1 == j {
                            triggered = true;
                            break;
                        } else if *ci > 0 && *cj > 0 && *ci - 1 == i && *cj - 1 == j {
                            triggered = true;
                            break;
                        } else if *ci > 0 && *ci - 1 == i && *cj + 1 == j {
                            triggered = true;
                            break;
                        } else if *cj > 0 && *ci + 1 == i && *cj - 1 == j {
                            triggered = true;
                            break;
                        }
                    }
                    if triggered {
                        buffer_cord.push(cord_set.clone());
                    }
                }
                star_coordinates.push(buffer_cord);
            }
        }
    }

    println!("Coordinates : {:?}", star_coordinates);
    let star_numbers = star_coordinates
        .iter()
        .map(|c| get_value_from_coordinates(&c, &lines))
        .filter(|c| c.len() > 1)
        .collect::<Vec<Vec<usize>>>();
    println!("Numbers : {:?}", star_numbers);
    println!(
        "Sum : {}",
        star_numbers
            .iter()
            .map(|c| c.iter().copied().reduce(|a, b| a * b).unwrap())
            .sum::<usize>()
    );

    let flatten_coordinates = star_coordinates
        .iter()
        .filter(|c| c.len() > 1)
        .flatten()
        .flatten()
        .collect::<Vec<&(usize, usize)>>();
    let mut new_lines: Vec<String> = Vec::new();
    for (i, line) in lines.into_iter().enumerate() {
        let mut new_line = line.to_string();
        for (ci, cj) in &flatten_coordinates {
            if i == *ci {
                new_line.remove(*cj);
                new_line.insert(*cj, 'X');
            }
        }
        new_lines.push(new_line);
    }
    println!("{}", new_lines.join("\n"));
}

fn get_value_from_coordinates(cord: &Vec<Vec<(usize, usize)>>, lines: &Vec<&str>) -> Vec<usize> {
    let numbers = cord
        .iter()
        .map(|c| {
            let mut num = String::new();
            for (i, j) in c {
                num.push(get_value(*i, *j, &lines));
            }
            num
        })
        .collect::<Vec<String>>()
        .iter()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    numbers
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
