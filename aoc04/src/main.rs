use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn check_fully_contains(first: &HashSet<i32>, second: &HashSet<i32>) -> bool {
    return first.is_subset(second) || second.is_subset(first);
}

fn check_overlaps(first: &HashSet<i32>, second: &HashSet<i32>) -> bool {
    return !first.is_disjoint(second);
}

fn part_one(data: &Vec<Vec<HashSet<i32>>>) -> i32 {
    let mut counter = 0;
    for team in data {
        if check_fully_contains(&team[0], &team[1]) {
            counter += 1;
        }
    }
    return counter;
}

fn part_two(data: &Vec<Vec<HashSet<i32>>>) -> i32 {
    let mut counter = 0;
    for team in data {
        if check_overlaps(&team[0], &team[1]) {
            counter += 1;
        }
    }
    return counter;
}

fn main() {
    let all_data = parse_data();
    let sum = part_one(&all_data);
    println!("Solution first part: {}", sum);

    let sum = part_two(&all_data);
    println!("Solution second part: {}", sum);
}

fn parse_data() -> Vec<Vec<HashSet<i32>>> {
    let mut result = Vec::new();
    let lines = read_lines("input").unwrap();
    for line in lines {
        if let Ok(linedata) = line {
            let elves: Vec<&str> = linedata.split(",").collect();
            assert!(elves.len() == 2);
            let mut result_elves = Vec::new();
            for elve in elves {
                let elve: Vec<&str> = elve.split("-").collect();
                let first = elve[0].parse::<i32>().unwrap();
                let last = elve[1].parse::<i32>().unwrap();
                let elve = first..=last;
                let elve: HashSet<i32> = HashSet::from_iter(elve);
                result_elves.push(elve);
            }
            result.push(result_elves);
        }
    }
    return result;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
