use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn part_one(stacks: &Vec<Stack>, commands: &Vec<Command>) -> String {
    let mut stacks = stacks.clone();
    for command in commands {
        for _ in 0..command.0 {
            let element = stacks[command.1].pop().unwrap();
            stacks[command.2].push(element);
        }
    }
    let result = stacks.iter().map(|stack| stack.last().unwrap()).collect();
    return result;
}

fn part_two(stacks: &Vec<Stack>, commands: &Vec<Command>) -> String {
    let mut stacks = stacks.clone();
    for command in commands {
        let mut tmp: Stack = Vec::new();
        for _ in 0..command.0 {
            let element = stacks[command.1].pop().unwrap();
            tmp.push(element);
        }
        tmp.reverse();
        for element in tmp {
            stacks[command.2].push(element);
        }
    }
    let result = stacks.iter().map(|stack| stack.last().unwrap()).collect();
    return result;
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

type Command = (i32, usize, usize);
type StackElement = char;
type Stack = Vec<StackElement>;

fn parse_stacks(stacks: &mut Vec<Stack>, line: &str) {
    let elements: Vec<char> = line.chars().collect();
    for index in 0..9 {
        let char_index = (index * 4) + 1;
        let element = elements[char_index];
        if element == '1' {
            break;
        }
        if element != ' ' {
            stacks[index].insert(0, element);
        }
    }
}

fn parse_command(commands: &mut Vec<Command>, line: &str) {
    let mut tokens = line.split(' ');
    tokens.next();
    let amount = tokens.next().unwrap().parse::<i32>().unwrap();
    tokens.next();
    let source = tokens.next().unwrap().parse::<usize>().unwrap() - 1;
    tokens.next();
    let target = tokens.next().unwrap().parse::<usize>().unwrap() - 1;
    commands.push((amount, source, target));
}

fn parse_input() -> (Vec<Stack>, Vec<Command>) {
    let mut stacks = vec![Vec::new(); 9];
    let mut commands = Vec::new();
    let lines = read_lines("input").unwrap();
    let mut mode = ParseMode::Stacks;

    for line in lines {
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
