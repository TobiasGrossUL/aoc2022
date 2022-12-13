use std::cell::RefCell;
use std::rc::Rc;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::rc::Weak;


fn main() {
    part_one();
    part_two();
}
fn is_right_order(left: &Packet, right: &Packet) -> Option<bool> {
    match (left, right) {
        (Packet::Item(left), Packet::Item(right)) => {
            if left < right {
                return Some(true);
            } else if right < left {
                return Some(false);
            } else {
                return None;
            }
        },
        (Packet::List(left), Packet::List(right)) => {
            for (left, right) in left.iter().zip(right.iter()) {
                let result = is_right_order(left, right);
                if result.is_some() {
                    return result;
                }
            }
            if left.len() < right.len() {
                return Some(true);
            } else if right.len() < left.len() {
                return Some(false);
            } else {
                return None;
            }
        }
        (Packet::Item(left), Packet::List(right)) => {
            let left = vec![Packet::Item(*left)];
            for (left, right) in left.iter().zip(right.iter()) {
                let result = is_right_order(left, right);
                if result.is_some() {
                    return result;
                }
            }
            if left.len() < right.len() {
                return Some(true);
            } else if right.len() < left.len() {
                return Some(false);
            } else {
                return None;
            }
        },
        (Packet::List(left), Packet::Item(right)) => {
            let right = vec![Packet::Item(*right)];
            for (left, right) in left.iter().zip(right.iter()) {
                let result = is_right_order(left, right);
                if result.is_some() {
                    return result;
                }
            }
            if left.len() < right.len() {
                return Some(true);
            } else if right.len() < left.len() {
                return Some(false);
            } else {
                return None;
            }
        },
    }
}

fn part_one() -> usize {
    let input = parse_input("input");
    let mut sum = 0;
    for (i, (left, right)) in input.iter().enumerate() {
        if is_right_order(&left, &right).unwrap() {
            sum += i + 1;
        }
    }
    println!("Solution part1: {}", sum);
    return sum;
}

fn part_two() -> usize {
    let result = 0;
    println!("Solution part2: {}", result);
    return result;
}

#[derive(Debug)]
enum Packet {
    List(Vec<Packet>),
    Item(i64)
}

struct AstNode {
    children: Vec<Rc<RefCell<AstNode>>>,
    parent: Weak<RefCell<AstNode>>,
    node_type: Packet,
}

impl AstNode {
    fn new() -> AstNode {
        AstNode {children: Vec::new(), parent: Weak::new(), node_type: Packet::List(Vec::new())}
    }
}

struct Parser {
    root: Rc<RefCell<AstNode>>,
    current_node: Weak<RefCell<AstNode>>,
}

#[derive(Debug)]
enum Token {
    LeftBrace,
    RightBrace,
    Number(i64),
}

impl Parser {
    fn new() -> Parser {
        let root = AstNode::new();
        let rootrc = Rc::new(RefCell::new(root));
        let current_node =  Rc::downgrade(&rootrc);
        Parser {root: rootrc, current_node}
    }

    fn _to_number(chars: Vec<char>) -> Token {
        let number_string: String = chars.into_iter().collect();
        let number = number_string.parse::<i64>().unwrap();
        return Token::Number(number);
    }

    fn _tokenize(char_input: Vec<char>) -> Vec<Token> {
        let mut result = Vec::new();
        let mut number_buffer = Vec::new();
        for char in char_input {
            match char {
                '[' => {
                    result.push(Token::LeftBrace);
                },
                ']' => {
                    if !number_buffer.is_empty() {
                        result.push(Self::_to_number(number_buffer));
                        number_buffer = Vec::new();
                    }
                    result.push(Token::RightBrace);
                },
                ',' => {
                    if !number_buffer.is_empty() {
                        result.push(Self::_to_number(number_buffer));
                        number_buffer = Vec::new();
                    }
                },
                _ => {
                    number_buffer.push(char);
                }
            }
        }
        return result;
    }

    fn _climb_down(&mut self) {
        let node_rc = self.current_node.upgrade().unwrap();
        let mut node_mut = node_rc.borrow_mut();
        let mut child_node = AstNode::new();
        child_node.parent = self.current_node.clone();
        child_node.node_type = Packet::List(Vec::new());
        node_mut.children.push(Rc::new(RefCell::new(child_node)));
        self.current_node = Rc::downgrade(&node_mut.children.last().unwrap());
    }

    fn _climb_up(&mut self) {
        let node_rc = self.current_node.upgrade().unwrap();
        let node = node_rc.borrow();
        self.current_node = node.parent.clone();
    }

    fn _add_value(&mut self, value: i64) {
        let node_rc = self.current_node.upgrade().unwrap();
        let mut node_mut = node_rc.borrow_mut();
        let mut child_node = AstNode::new();
        child_node.parent = self.current_node.clone();
        child_node.node_type = Packet::Item(value);
        node_mut.children.push(Rc::new(RefCell::new(child_node)));
    }

    fn _build_packet_struct(current_node: &AstNode) -> Packet {
        match current_node.node_type {
            Packet::Item(value) => return Packet::Item(value),
            Packet::List(_) => {
                let mut result = Vec::new();
                for child in current_node.children.iter() {
                    let p = Self::_build_packet_struct(&child.borrow());
                    result.push(p);
                }
                return Packet::List(result);
            },
        }
    }

    fn parse(&mut self, line: &str) -> Packet {
        let mut input = Self::_tokenize(line.chars().collect());
        // delete first array tokens because we have allready created root list
        input.pop();
        input.remove(0);

        for token in input.iter() {
            match token {
                Token::LeftBrace => self._climb_down(),
                Token::RightBrace => self._climb_up(),
                Token::Number(value) => self._add_value(*value),
            }
        }
        let r = self.root.borrow();
        return Self::_build_packet_struct(&r);
    }

}

fn parse_input(filename: &str) -> Vec<(Packet, Packet)> {
    let mut result = Vec::new();
    let mut line_storage = Vec::new();
    for line in read_lines(filename).unwrap() {
        if let Ok(linedata) = line {
            if linedata == "" {
                let first = line_storage.remove(0);
                let second = line_storage.remove(0);
                result.push((first, second));
                continue;
            }
            let mut parser = Parser::new();
            let linepacket = parser.parse(&linedata);
            line_storage.push(linepacket);
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
    fn part_one_test() {
        let solution = part_one();
        assert_eq!(solution, 0);
    }

    #[test]
    fn part_two_test() {
        let solution = part_two();
        assert_eq!(solution, 0);
    }
}
