use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn part_one(stacks: &Vec<Stack>, commands: &Vec<Command>) -> String {
    let mut stacks = stacks.clone();
    for command in commands {
        for _ in 0..command.amount {
            let element = stacks[command.src].pop().unwrap();
            stacks[command.dest].push(element);
        }
    }
    return stacks.iter().map(|stack| stack.last().unwrap()).collect();
}

fn part_two(stacks: &Vec<Stack>, commands: &Vec<Command>) -> String {
    let mut stacks = stacks.clone();
    for command in commands {
        let mut crane: Stack = Vec::new();
        for _ in 0..command.amount {
            let element = stacks[command.src].pop().unwrap();
            crane.push(element);
        }
        crane.reverse();
        stacks[command.dest].append(&mut crane);
    }
    return stacks.iter().map(|stack| stack.last().unwrap()).collect();
}

fn main() {
    let (stacks, commands)  = parse_input();

    let sol1 = part_one(&stacks, &commands);
    println!("Solution part 1: {}", sol1);

    let sol2 = part_two(&stacks, &commands);
    println!("Solution part 2: {}", sol2);
}

enum ParseMode {
    Stacks,
    Commands,
}

struct Command {
    amount: i32,
    src: usize,
    dest: usize
}

type StackElement = char;
type Stack = Vec<StackElement>;

fn parse_stacks(stacks: &mut Vec<Stack>, line: &str) {
    if line.contains('1') {
        return;
    }

    for (index, element) in line.chars().enumerate() {
        if !element.is_alphabetic() {
            continue;
        }
        let stack_index = (index - 1) / 4;
        stacks[stack_index].insert(0, element);
    }
}

fn parse_command(commands: &mut Vec<Command>, line: &str) {
    let mut command = Command {amount:0, src:0, dest:0};
    for (index, token) in line.split(' ').enumerate() {
        match index {
            1 => command.amount = token.parse::<i32>().unwrap(),
            3 => command.src = token.parse::<usize>().unwrap() - 1,
            5 => command.dest = token.parse::<usize>().unwrap() - 1,
            _ => ()
        }
    }
    commands.push(command);
}

fn parse_input() -> (Vec<Stack>, Vec<Command>) {
    let mut stacks = vec![Vec::new(); 9];
    let mut commands = Vec::new();
    let mut mode = ParseMode::Stacks;

    for line in read_lines("input").unwrap() {
        if let Ok(linedata) = line {
            if linedata == "" {
                mode = ParseMode::Commands;
                continue;
            }
            match mode {
                ParseMode::Stacks => parse_stacks(&mut stacks, &linedata),
                ParseMode::Commands => parse_command(&mut commands, &linedata)
            }
        }
    }
    return (stacks, commands);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
