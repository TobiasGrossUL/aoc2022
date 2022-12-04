use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn part_one() -> i32 {
    let all_data = parse_data();
    return 0;
}

fn main() {
    let sum = part_one();
    println!("Solution first part: {}", sum);
}

fn parse_data() -> Vec<(HashSet<char>, HashSet<char>)> {
    let mut result = Vec::new();
    let lines = read_lines("input").unwrap();
    for line in lines {
        if let Ok(linedata) = line {
            let chars: Vec<char> = linedata.chars().collect();
            let half = chars.len() /2;
            let compartment1 = &chars[..half];
            let compartment2 = &chars[half..];
            let compartment1 = HashSet::from_iter(compartment1.iter().cloned());
            let compartment2 = HashSet::from_iter(compartment2.iter().cloned());
            result.push((compartment1, compartment2));
        }
    }
    return result;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
