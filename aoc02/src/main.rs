use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main_old() {
    let mut max_callories = Vec::new();
    let mut current_callories = 0;
    let lines = read_lines("input").unwrap();
    for line in lines {
        if let Ok(callories) = line {
            match callories.parse::<i32>() {
                Ok(cal) => {
                    current_callories += cal;
                },
                Err(_) => {
                    max_callories.push(current_callories);
                    current_callories = 0;
                },
            }

        }
    }
    max_callories.push(current_callories);
    max_callories.sort();
    max_callories.reverse();
    println!("{}, {}, {}", max_callories[0], max_callories[1], max_callories[2]);
    let top_three = max_callories[..3].iter().fold(0, |mut sum, &x| {sum += x; sum});
    println!("{}", top_three);
}

fn main() {

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
