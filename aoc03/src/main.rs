use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn map_item(item: char) -> i32 {
    if item.is_uppercase() {
        return ((item as u32) - ('A' as u32) + 27) as i32;
    } else {
        return ((item as u32) - ('a' as u32) + 1) as i32;
    }
}

fn part_one() -> i32 {
    let all_data = parse_data();
    let mut sum = 0;
    for rucksack in all_data {
        let double_item = rucksack.0.intersection(&rucksack.1).into_iter().next().unwrap();
        let value = map_item(*double_item);
        sum += value;
    }
    return sum;
}

fn merge_set<'a>(set1: &'a HashSet<char>, set2: &'a HashSet<char>) -> HashSet<&'a char>{
    let mut target: HashSet<&char> = HashSet::new();
    target.extend(set1);
    target.extend(set2);
    return target;
}

fn intersect<'a>(sets: &'a [HashSet<&char>]) -> char {
    let mut result: HashSet<&char> = HashSet::new();
    for (i, set) in sets.iter().enumerate() {
        if i == 0 {
            result.extend(set);
        } else {
            result = result.intersection(set).copied().collect();
        }
    }
    return **result.iter().next().unwrap();
}

fn part_two() -> i32 {
    let all_data = parse_data();
    let mut sum = 0;
    let no_comps : Vec<HashSet<&char>> = all_data.iter().map(|rucksack| {merge_set(&rucksack.0, &rucksack.1)}).collect();
    for group in no_comps.chunks(3) {
        let value = intersect(group);
        let value = map_item(value);
        sum += value;
    }
    return sum;
}

fn main() {
    let sum = part_one();
    println!("Solution first part: {}", sum);

    let sum = part_two();
    println!("Solution second part: {}", sum);
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
