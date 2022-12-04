use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn check_fully_contains(first_elve: &HashSet<i32>, second_elve: &HashSet<i32>) -> bool {
    return first_elve.is_subset(second_elve) || second_elve.is_subset(first_elve);
}

fn check_overlaps(first_elve: &HashSet<i32>, second_elve: &HashSet<i32>) -> bool {
    return !first_elve.is_disjoint(second_elve);
}

fn part_one(camp_sections: &Vec<(HashSet<i32>, HashSet<i32>)>) -> usize {
    let fully_overlapping = camp_sections.iter().filter(|team| {check_fully_contains(&team.0, &team.1)}).count();
    return fully_overlapping;
}

fn part_two(camp_sections: &Vec<(HashSet<i32>, HashSet<i32>)>) -> usize {
    let partially_overlapping = camp_sections.iter().filter(|team| {check_overlaps(&team.0, &team.1)}).count();
    return partially_overlapping;
}

fn main() {
    let camp_sections = parse_camp_sections();
    let sum = part_one(&camp_sections);
    println!("Solution first part: {}", sum);

    let sum = part_two(&camp_sections);
    println!("Solution second part: {}", sum);
}

fn make_elve_space(data_repr: &str) -> HashSet<i32> {
    let elve_space: Vec<&str> = data_repr.split("-").collect();
    let first = elve_space[0].parse::<i32>().unwrap();
    let last = elve_space[1].parse::<i32>().unwrap();
    let elve_space = first..=last;
    let elve_space: HashSet<i32> = HashSet::from_iter(elve_space);
    return elve_space;
}

fn parse_camp_sections() -> Vec<(HashSet<i32>, HashSet<i32>)> {
    let mut result = Vec::new();
    let lines = read_lines("input").unwrap();
    for line in lines {
        if let Ok(team_data) = line {
            let elves: Vec<&str> = team_data.split(",").collect();
            assert!(elves.len() == 2);
            let first_space = make_elve_space(elves[0]);
            let second_space = make_elve_space(elves[1]);
            result.push((first_space, second_space));
        }
    }
    return result;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
