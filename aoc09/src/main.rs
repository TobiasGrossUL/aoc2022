use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

#[derive(Clone)]
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
    knots: Vec<Coordinate>,
    tail_positions: HashSet<(i32, i32)>
}

impl Rope {
    fn new(knot_count: usize) -> Rope {
        let knots = vec![Coordinate::new(0,0); knot_count];
        let tail_positions = HashSet::new();
        Rope {knots, tail_positions}
    }

    fn _move_head(&mut self, com: &Command) {
        match com {
            Command::Up(_) => self.knots[0].change_y(1),
            Command::Down(_) => self.knots[0].change_y(-1),
            Command::Right(_) => self.knots[0].change_x(1),
            Command::Left(_) => self.knots[0].change_x(-1)
        }
    }

    fn _move_tail(&mut self) {
        let indeces: Vec<usize> = (0..self.knots.len()).collect();
        for indeces in indeces.windows(2) {
            //self.visualize(10, 10, "int");
            let head = self.knots[indeces[0]].clone();
            let distance = head.diff(&self.knots[indeces[1]]);
            let tail = &mut self.knots[indeces[1]];
            match distance {
                (2, 0) => tail.change_x(-1),
                (-2, 0) => tail.change_x(1),
                (0, 2) => tail.change_y(-1),
                (0, -2) =>tail.change_y(1),
                (2, 1) => {
                    tail.change_x(-1);
                    tail.change_y(-1);
                },
                (2, -1) => {
                    tail.change_x(-1);
                    tail.change_y(1);
                },
                (-2, 1) => {
                    tail.change_x(1);
                    tail.change_y(-1);
                },
                (-2, -1) => {
                    tail.change_x(1);
                    tail.change_y(1);
                },
                (1, 2) => {
                    tail.change_x(-1);
                    tail.change_y(-1);
                },
                (-1, 2) => {
                    tail.change_x(1);
                    tail.change_y(-1);
                },
                (1, -2) => {
                    tail.change_x(-1);
                    tail.change_y(1);
                },
                (-1, -2) => {
                    tail.change_x(1);
                    tail.change_y(1);
                },
                (2, 2) => {
                    tail.change_x(-1);
                    tail.change_y(-1);
                }
                (2, -2) => {
                    tail.change_x(-1);
                    tail.change_y(1);
                }
                (-2, 2) => {
                    tail.change_x(1);
                    tail.change_y(-1);
                }
                (-2, -2) => {
                    tail.change_x(1);
                    tail.change_y(1);
                }
                _ => ()
            }
        }
    }

    fn visualize(&self, height: usize, width: usize, command: &str) {
        println!("====={}", command);
        let mut points = vec![vec![String::from("."); width]; height];
        for (i, knot) in self.knots.iter().enumerate().rev() {
            let token;
            if i == 0 {
                token = String::from("H");
            } else {
                token = (i).to_string();
            }

            points[knot.y as usize][knot.x as usize] = token;
        }
        for line in points.iter().rev() {
            let aline = line.join("");
            println!("{}", aline);
        }
        println!("");
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
            self.tail_positions.insert((self.knots.last().unwrap().x, self.knots.last().unwrap().y));
        }
    }

    fn get_amount_tail_positions(&self) -> usize {
        return self.tail_positions.len();
    }
}

fn main() {
    let commands = parse_input();
    part_one(&commands);
    part_two(&commands);
    //test();
}

fn test() {
    let mut rope = Rope::new(10);
    rope.exec_command(&Command::Right(5));
    rope.visualize(10, 10, "R5");

    rope.exec_command(&Command::Up(8));
    rope.visualize(10, 10, "U8");
}

fn part_one(commands: &Vec<Command>) {
    let mut rope = Rope::new(2);
    for command in commands {
        rope.exec_command(command);
    }
    let postitions = rope.get_amount_tail_positions();
    println!("Solution part 1 {}", postitions);
}

fn part_two(commands: &Vec<Command>) {
    let mut rope = Rope::new(10);
    for command in commands {
        rope.exec_command(command);
    }
    let postitions = rope.get_amount_tail_positions();
    println!("Solution part 2 {}", postitions);
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
        let mut rope = Rope::new(2);
        rope.exec_command(&Command::Up(1));
        assert_eq!(1, rope.knots[0].y);
        assert_eq!(0, rope.knots[0].x);
        assert_eq!(0, rope.knots[1].y);
        assert_eq!(0, rope.knots[1].x);

        rope.exec_command(&Command::Up(1));
        assert_eq!(2, rope.knots[0].y);
        assert_eq!(0, rope.knots[0].x);
        assert_eq!(1, rope.knots[1].y);
        assert_eq!(0, rope.knots[1].x);

        rope.exec_command(&Command::Up(2));
        assert_eq!(4, rope.knots[0].y);
        assert_eq!(0, rope.knots[0].x);
        assert_eq!(3, rope.knots[1].y);
        assert_eq!(0, rope.knots[1].x);
    }

    #[test]
    fn head_up_2() {
        let mut rope = Rope::new(3);
        rope.exec_command(&Command::Up(1));
        assert_eq!(1, rope.knots[0].y);
        assert_eq!(0, rope.knots[0].x);
        assert_eq!(0, rope.knots[1].y);
        assert_eq!(0, rope.knots[1].x);
        assert_eq!(0, rope.knots[2].y);
        assert_eq!(0, rope.knots[2].x);

        rope.exec_command(&Command::Up(1));
        assert_eq!(2, rope.knots[0].y);
        assert_eq!(0, rope.knots[0].x);
        assert_eq!(1, rope.knots[1].y);
        assert_eq!(0, rope.knots[1].x);
        assert_eq!(0, rope.knots[2].y);
        assert_eq!(0, rope.knots[2].x);

        rope.exec_command(&Command::Up(2));
        assert_eq!(4, rope.knots[0].y);
        assert_eq!(0, rope.knots[0].x);
        assert_eq!(3, rope.knots[1].y);
        assert_eq!(0, rope.knots[1].x);
        assert_eq!(2, rope.knots[2].y);
        assert_eq!(0, rope.knots[2].x);
    }
}
