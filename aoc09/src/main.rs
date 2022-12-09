use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

struct Coordinate {
    x: i32,
    y: i32
}

impl Coordinate {
    fn new(x: i32, y: i32) -> Coordinate {
        Coordinate { x, y }
    }

    fn change_x(&mut self, x: i32) {
        self.x += x;
    }

    fn change_y(&mut self, y: i32) {
        self.y += y;
    }

    fn diff(&self, other: &Coordinate) -> (i32, i32) {
        let x_diff = other.x - self.x;
        let y_diff = other.y - self.y;
        return (x_diff, y_diff);
    }
}

struct Rope {
    head: Coordinate,
    tail: Coordinate,
    tail_positions: HashSet<(i32, i32)>
}

impl Rope {
    fn new() -> Rope {
        let head = Coordinate::new(0,0);
        let tail = Coordinate::new(0,0);
        let tail_positions = HashSet::new();
        Rope {head, tail, tail_positions}
    }

    fn _move_head(&mut self, com: &Command) {
        match com {
            Command::Up(_) => self.head.change_y(1),
            Command::Down(_) => self.head.change_y(-1),
            Command::Right(_) => self.head.change_x(1),
            Command::Left(_) => self.head.change_x(-1)
        }
    }

    fn _move_tail(&mut self) {
        let distance = self.head.diff(&self.tail);
        match distance {
            (2, 0) => self.tail.change_x(-1),
            (-2, 0) => self.tail.change_x(1),
            (0, 2) => self.tail.change_y(-1),
            (0, -2) => self.tail.change_y(1),
            (2, 1) => {
                self.tail.change_x(-1);
                self.tail.change_y(-1);
            },
            (2, -1) => {
                self.tail.change_x(-1);
                self.tail.change_y(1);
            },
            (-2, 1) => {
                self.tail.change_x(1);
                self.tail.change_y(-1);
            },
            (-2, -1) => {
                self.tail.change_x(1);
                self.tail.change_y(1);
            },
            (1, 2) => {
                self.tail.change_x(-1);
                self.tail.change_y(-1);
            },
            (-1, 2) => {
                self.tail.change_x(1);
                self.tail.change_y(-1);
            },
            (1, -2) => {
                self.tail.change_x(-1);
                self.tail.change_y(1);
            },
            (-1, -2) => {
                self.tail.change_x(1);
                self.tail.change_y(1);
            },
            _ => ()
        }
    }

    fn exec_command(&mut self, com: &Command) {
        match com {
            Command::Up(len) => self._exec_command(com, *len),
            Command::Down(len) => self._exec_command(com, *len),
            Command::Left(len) => self._exec_command(com, *len),
            Command::Right(len) => self._exec_command(com, *len)
        }
    }

    fn _exec_command(&mut self, com: &Command, times:i32) {
        for _ in 0..times {
            self._move_head(com);
            self._move_tail();
            self.tail_positions.insert((self.tail.x, self.tail.y));
        }
    }

    fn get_amount_tail_positions(&self) -> usize {
        return self.tail_positions.len();
    }
}

fn main() {
    let commands = parse_input();
    part_one(&commands);
}

fn part_one(commands: &Vec<Command>) {
    let mut rope = Rope::new();
    for command in commands {
        rope.exec_command(command);
    }
    let postitions = rope.get_amount_tail_positions();
    println!("Solution part 1 {}", postitions);
}

enum Command {
    Up(i32),
    Down(i32),
    Right(i32),
    Left(i32)
}


fn parse_input() -> Vec<Command> {
    let mut result = Vec::new();
    for line in read_lines("input").unwrap() {
        if let Ok(linedata) = line {
            let mut tokens = linedata.split_whitespace();
            let direction = tokens.nth(0).unwrap();
            let length = tokens.nth(0).unwrap().parse::<i32>().unwrap();
            let command = match direction {
                "L" => Command::Left(length),
                "R" => Command::Right(length),
                "U" => Command::Up(length),
                "D" => Command::Down(length),
                _ => panic!("Unknown command")
            };
            result.push(command);
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
    fn head_up() {
        let mut rope = Rope::new();
        rope.exec_command(&Command::Up(1));
        assert_eq!(1, rope.head.y);
        assert_eq!(0, rope.head.x);
        assert_eq!(0, rope.tail.y);
        assert_eq!(0, rope.tail.x);

        rope.exec_command(&Command::Up(1));
        assert_eq!(2, rope.head.y);
        assert_eq!(0, rope.head.x);
        assert_eq!(1, rope.tail.y);
        assert_eq!(0, rope.tail.x);

        rope.exec_command(&Command::Up(2));
        assert_eq!(4, rope.head.y);
        assert_eq!(0, rope.head.x);
        assert_eq!(3, rope.tail.y);
        assert_eq!(0, rope.tail.x);
    }
}
