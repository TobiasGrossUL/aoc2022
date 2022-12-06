use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn main() {
    let input = parse_input().unwrap();
    let position = find_marker(&input, 4).unwrap();
    println!("Solution part 1: {}", position);

    let position = find_marker(&input, 14).unwrap();
    println!("Solution part 2: {}", position);
}

fn check_marker(marker: &[char], size: usize) -> bool {
    if marker.len() != size {
        return false;
    }
    let token_set: HashSet<&char> = HashSet::from_iter(marker);
    return token_set.len() == size;
}

fn find_marker(tokens: &Vec<char>, size: usize) -> Option<usize> {
    for i in 0..tokens.len() {
        let marker_slice = &tokens[i..i+size];
        let is_marker = check_marker(marker_slice, size);
        if is_marker {
            return Some(i + size);
        }
    }
    return None;
}

fn parse_input() -> Option<Vec<char>> {
    for line in read_lines("input").unwrap() {
        if let Ok(linedata) = line {
            let tokens = linedata.chars().collect();
            return Some(tokens);
        }
    }
    return None;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
