use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn map_item(item: &char) -> i32 {
    let value = item.clone() as u32;
    if value < 91 {
        return (value - 64 + 26) as i32;
    } else {
        return (value - 96) as i32;
    }
}

fn main() {
    let all_data = parse_data();
    let mut sum = 0;
    for rucksack in all_data {
        let double_item = rucksack.0.intersection(&rucksack.1).collect::<Vec<&char>>()[0];
        let value = map_item(double_item);
        sum += value;
    }
    println!("Solution first part: {}", sum);


    let all_data = parse_data();
    let mut sum = 0;
    let mut secret_badge1: HashSet<&char> = HashSet::new();
    let mut secret_badge2: HashSet<&char> = HashSet::new();
    let mut secret_badge3: HashSet<&char> = HashSet::new();
    for (i, rucksack) in all_data.iter().enumerate() {
        let phase = i % 3;
        match phase {
            0 => {
                secret_badge1.extend(&rucksack.0);
                secret_badge1.extend(&rucksack.1);
            },
            1 => {
                secret_badge2.extend(&rucksack.0);
                secret_badge2.extend(&rucksack.1);
            },
            2 => {
                secret_badge3.extend(&rucksack.0);
                secret_badge3.extend(&rucksack.1);

                // let first_intersetctiojkksecret_badge1.intersection(&secret_badge2).copied.collect::<HashSet<&&char>>().intersection(&secret_badge3);
                let first: HashSet<&char> = secret_badge1.intersection(&secret_badge2).copied().collect();
                let first: Vec<&char> = first.intersection(&secret_badge3).copied().collect();
                secret_badge1.clear();
                secret_badge2.clear();
                secret_badge3.clear();
                let value = map_item(first[0]);
                sum += value;
            },
            _ => {
                println!{"Error"};
            }
        }
    }
    println!("Solution second part: {}", sum);
}

fn parse_data() -> Vec<(HashSet<char>, HashSet<char>)> {
    let mut result = Vec::new();
    let lines = read_lines("input").unwrap();
    for line in lines {
        if let Ok(linedata) = line {
            let chars: Vec<char> = linedata.chars().collect();
            let half = chars.len() /2;
            let mut comp1 = HashSet::new();
            let mut comp2 = HashSet::new();
            for (i, x) in chars.iter().enumerate() {
                if i < half {
                    comp1.insert(x.clone());
                } else {
                    comp2.insert(x.clone());
                }
            }
            result.push((comp1, comp2));
        }
    }
    return result;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
