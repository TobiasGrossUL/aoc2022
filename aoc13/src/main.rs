use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    part_one();
    part_two();
}

fn part_one() -> usize {
    let result = 0;
    println!("Solution part1: {}", result);
    return result;
}

fn part_two() -> usize {
    let result = 0;
    println!("Solution part2: {}", result);
    return result;
}

fn parse_input(filename: &str) -> Map {
    let mut result = Vec::new();
    let mut start_index = (0, 0);
    let mut target_index = (0, 0);
    for (row_index, line) in read_lines(filename).unwrap().enumerate() {
        if let Ok(linedata) = line {
            let mut row = Vec::new();
            for (column_index, element) in linedata.chars().enumerate() {
                let elevation = match element {
                    'S' => {start_index = (row_index, column_index); 0},
                    'E' => {target_index = (row_index, column_index); 'z' as u32 - 'a' as u32},
                    _ => {element as u32 - 'a' as u32},
                };
                row.push(elevation);
            }
            result.push(row);
        }
    }
    return Map::new(start_index, target_index, result);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        let solution = part_one();
        assert_eq!(solution, 0);
    }

    #[test]
    fn part_two_test() {
        let solution = part_two();
        assert_eq!(solution, 0);
    }
}
