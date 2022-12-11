use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::SplitWhitespace;
use std::cell::RefCell;

struct Monkey {
    items: Vec<i64>,
    item_operation: (Operation, Option<i64>),
    test: i64,
    test_true: usize,
    test_false: usize,
    inspections: i64
}

impl Monkey {
    fn new(parts: &mut Vec<Part>) -> Monkey {
        let mut items = Vec::new();
        let mut item_operation = (Operation::Add, None);
        let mut test = 0;
        let mut test_true = 0;
        let mut test_false = 0;
        let inspections = 0;
        loop {
            if parts.is_empty() {
                break;
            }
            let part = parts.remove(0);
            match part {
                Part::StartItem(item) => items.push(item),
                Part::Operation(op) => item_operation = op,
                Part::Test(cond) => test = cond,
                Part::TestTrue(target) => test_true = target,
                Part::TestFalse(target) => test_false = target,
                Part::Monkey => break
            };

        }
        Monkey {items, item_operation, test, test_true, test_false, inspections}
    }

    fn receive_item(&mut self, item: i64) {
        self.items.push(item);
    }

    fn inspect_item(&self, item: i64) -> (i64, usize) {
        let  param = match self.item_operation.1 {
            Some(x) => x,
            None => item
        };
        let new_item = match self.item_operation.0 {
            Operation::Add => (item + param) / 3,
            Operation::Mult => (item * param) / 3
        };

        if new_item % self.test == 0 {
            return (new_item, self.test_true);
        } else {
            return (new_item, self.test_false);
        }
    }

    fn throw_item(item: i64, reveiver: usize, monkeys: &Vec<RefCell<Monkey>>) {
        monkeys[reveiver].borrow_mut().receive_item(item);
    }

    fn do_monkey_business(&mut self, monkeys: &Vec<RefCell<Monkey>>) {
        while !self.items.is_empty() {
            let item = self.items.remove(0);
            self.inspections += 1;
            let (item, receiver) = self.inspect_item(item);
            Self::throw_item(item, receiver, monkeys);
        }

    }
}

fn build_monkeys(parts: &mut Vec<Part>) -> Vec<RefCell<Monkey>> {
    parts.remove(0);
    let mut result = Vec::new();
    while !parts.is_empty() {
        result.push(RefCell::new(Monkey::new(parts)));
    }
    return result;
}

fn main() {
    part_one();
    part_two();
}

fn monkey_business(monkeys: &Vec<RefCell<Monkey>>) -> i64 {
    let mut scores = Vec::new();
    for monkey in monkeys {
        scores.push(monkey.borrow().inspections);
    }
    scores.sort();
    scores.reverse();
    println!("scores: {:?}", scores);
    return scores[0] * scores[1];
}

fn part_one() -> i64 {
    let mut input = parse_input("input");
    let monkeys = build_monkeys(&mut input);
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            monkeys[i].borrow_mut().do_monkey_business(&monkeys);
        }
    }
    let solution = monkey_business(&monkeys);
    println!("Solution part1: {}", solution);
    return solution;
}

fn part_two() {
}

enum Part {
    StartItem(i64),
    Operation((Operation, Option<i64>)),
    Test(i64),
    TestTrue(usize),
    TestFalse(usize),
    Monkey
}

enum Operation {
    Add,
    Mult,
}

fn parse_starting_items(tokens: &mut SplitWhitespace) -> Vec<Part> {
    let mut result = Vec::new();
    let second = tokens.nth(0).unwrap();
    assert_eq!(second, "items:");

    for number in tokens {
        let number = String::from(number);
        let number = number.trim_end_matches(',');
        let number = number.parse::<i64>().unwrap();
        result.push(Part::StartItem(number));
    }
    return result;
}

fn parse_operation(tokens: &mut SplitWhitespace) -> Part {
    let second = tokens.nth(0).unwrap();
    assert_eq!(second, "new");
    tokens.nth(1);
    let operator = tokens.nth(0).unwrap();
    let operator = operator.chars().nth(0).unwrap();
    let operator = match operator {
        '*' => Operation::Mult,
        '+' => Operation::Add,
        _ => panic!("Can not be here")
    };
    let parameter = tokens.nth(0).unwrap();
    let parameter = parameter.parse::<i64>();
    if parameter.is_ok() {
        return Part::Operation((operator, Some(parameter.unwrap())));
    } else {
        return Part::Operation((operator, None));
    }
}

fn parse_test(tokens: &mut SplitWhitespace) -> Part {
    let second = tokens.nth(0).unwrap();
    assert_eq!(second, "divisible");
    tokens.nth(0);
    let divisor = tokens.nth(0).unwrap().parse::<i64>().unwrap();
    return Part::Test(divisor);
}

fn parse_if(tokens: &mut SplitWhitespace) -> Part {
    let which = tokens.nth(0).unwrap();
    tokens.nth(2); //throw to monkey
    let monkey =tokens.nth(0).unwrap().parse::<usize>().unwrap();
    match which {
        "true:" => Part::TestTrue(monkey),
        "false:" => Part::TestFalse(monkey),
        _ => panic!("Should not be here")
    }
}

fn parse_input(filename: &str) -> Vec<Part> {
    let mut result = Vec::new();
    for line in read_lines(filename).unwrap() {
        if let Ok(linedata) = line {
            if linedata == "" {
                continue;
            }
            let mut tokens = linedata.split_whitespace();
            let first_token = tokens.nth(0).unwrap();
            match first_token {
                "Starting" => result.append(&mut parse_starting_items(&mut tokens)),
                "Operation:" => result.push(parse_operation(&mut tokens)),
                "Test:" => result.push(parse_test(&mut tokens)),
                "If" => result.push(parse_if(&mut tokens)),
                "Monkey" => result.push(Part::Monkey),
                _ => panic!("Should not be here: {}", linedata)
            };
        }
    }
    return result;
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
    fn simple() {
    }

    #[test]
    fn complex() {
    }
}
