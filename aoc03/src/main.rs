use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Copy, Clone)]
enum Hand {
    Rock,
    Paper,
    Scissors
}

enum Outcome {
    Win,
    Draw, Loose
}

fn main() {
    let all_data = parse_data();
    let mut total_points1 = 0;
    let mut total_points2= 0;
    for data in all_data {
       total_points1 += points1(data.0, data.1);
       total_points2 += points2(data.0, data.2);
    }
    println!("Solution Part 1: {}", total_points1);
    println!("Solution Part 2: {}", total_points2);
}

fn get_win_hand(oponent: Hand) -> Hand {
    return match oponent {
        Hand::Rock => Hand::Paper,
        Hand::Paper => Hand::Scissors,
        Hand::Scissors => Hand::Rock,
    }
}

fn get_loose_hand(oponent: Hand) -> Hand {
    match oponent {
        Hand::Rock => Hand::Scissors,
        Hand::Paper => Hand::Rock,
        Hand::Scissors => Hand::Paper,
    }
}

fn points2(oponent: Hand, myself: Outcome) -> i32 {
    let play = match myself {
        Outcome::Loose => get_loose_hand(oponent),
        Outcome::Win => get_win_hand(oponent),
        Outcome::Draw => oponent.clone(),
    };
    return points1(oponent, play);
}

fn points1(oponent: Hand, myself: Hand) -> i32 {
    let shape_points = match myself {
        Hand::Rock => 1,
        Hand::Paper => 2,
        Hand::Scissors => 3,
    };

    let outcome = match (oponent, myself) {
        (Hand::Rock, Hand::Scissors) => 0,
        (Hand::Rock, Hand::Paper) => 6,
        (Hand::Scissors, Hand::Paper) => 0,
        (Hand::Scissors, Hand::Rock) => 6,
        (Hand::Paper, Hand::Rock) => 0,
        (Hand::Paper, Hand::Scissors) => 6,
        (_, _) => 3,
    };
    return shape_points + outcome;
}

fn get_hand(element: &str) -> Option<Hand> {
    match element {
        "A" => Some(Hand::Rock),
        "B" => Some(Hand::Paper),
        "C" => Some(Hand::Scissors),
        "X" => Some(Hand::Rock),
        "Y" => Some(Hand::Paper),
        "Z" => Some(Hand::Scissors),
        _ => None,
    }
}

fn get_outcome(element: &str) -> Option<Outcome> {
    match element {
        "X" => Some(Outcome::Loose),
        "Y" => Some(Outcome::Draw),
        "Z" => Some(Outcome::Win),
        _ => None,
    }
}

fn parse_data() -> Vec<(Hand, Hand, Outcome)> {
    let mut result = Vec::new();
    let lines = read_lines("input").unwrap();
    for line in lines {
        if let Ok(linedata) = line {
            let elements: Vec<&str> = linedata.split(" ").collect();
            let oponent = get_hand(elements[0]).unwrap();
            let myself = get_hand(elements[1]).unwrap();
            let outcome = get_outcome(elements[1]).unwrap();
            result.push((oponent, myself, outcome));
        }
    }
    return result;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
