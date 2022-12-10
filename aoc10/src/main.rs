use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct CPU {
    reg_x: i32,
    cycle: i32,
    command_stack: Vec<Command>,
    blocked: i32,
    current_command: Option<Command>,
    pixels: Vec<char>,
}

impl CPU {
    fn new(commands: Vec<Command>) -> CPU {
        let pixels: Vec<char> = vec!{'.'; (6 * 40) as usize};
        CPU {cycle: 1, reg_x: 1, command_stack: commands, blocked: 0, current_command: None, pixels}
    }

    fn execute_current_comannd(&mut self) {
        let the_command = self.current_command.as_ref().unwrap();
        match the_command {
            Command::Addx(amount) => self.reg_x += amount,
            _ => ()
        }
        self.current_command = None;
    }

    fn _draw(&mut self) {
        let index = self.cycle -1;
        let compare = index % 40;
        if compare >= self.reg_x - 1 && compare <= self.reg_x + 1 {
            self.pixels[index as usize] = '#';
        }
    }

    fn draw(&self) {
        for line in self.pixels.chunks(40) {
            let tmp: Vec<char> = line.iter().cloned().collect();
            let stringline: String = tmp.into_iter().collect();
            println!("{}", stringline);
        }
    }

    fn execute_next_command(&mut self) {
        let next_cmd = self.command_stack.remove(0);
        match next_cmd {
            Command::Addx(_) => {
                self.current_command = Some(next_cmd);
                self.blocked = 1;
            },
            Command::Noop() => ()
        };
        self.cycle += 1;
    }

    fn do_cycle (&mut self) -> (i32, i32) {
        self._draw();
        if self.blocked > 0 {
            self.blocked -= 1;
            self.cycle += 1;
            if self.blocked == 0 {
                self.execute_current_comannd();
            }
            return (self.reg_x, self.cycle);
        } else {
            self.execute_next_command();
        }

        return (self.reg_x, self.cycle);
    }
}

fn main() {
    part_one();
    part_two();
}

fn part_one() -> i32 {
    let commands = parse_input("input");
    let mut cpu = CPU::new(commands);
    let mut sum = 0;
    for _ in 1..20 {
        cpu.do_cycle();
    }
    sum = sum + cpu.cycle * cpu.reg_x;

    for _ in 0..40 {
        cpu.do_cycle();
    }
    sum = sum + cpu.cycle * cpu.reg_x;

    for _ in 0..40 {
        cpu.do_cycle();
    }
    sum = sum + cpu.cycle * cpu.reg_x;

    for _ in 0..40 {
        cpu.do_cycle();
    }
    sum = sum + cpu.cycle * cpu.reg_x;

    for _ in 0..40 {
        cpu.do_cycle();
    }
    sum = sum + cpu.cycle * cpu.reg_x;

    for _ in 0..40 {
        cpu.do_cycle();
    }
    sum = sum + cpu.cycle * cpu.reg_x;
    println!("Solution part1: {}", sum);

    return sum;
}

fn part_two() {
    let commands = parse_input("input");
    let mut cpu = CPU::new(commands);
    for _ in 1..240 {
        cpu.do_cycle();
    }
    cpu.draw();
}

enum Command {
    Noop(),
    Addx(i32)
}

fn parse_input(filename: &str) -> Vec<Command> {
    let mut result = Vec::new();
    for line in read_lines(filename).unwrap() {
        if let Ok(linedata) = line {
            let mut tokens = linedata.split_whitespace();
            let command_type = tokens.nth(0).unwrap();
            let parameter = tokens.nth(0);
            let command = match command_type {
                "addx" => Command::Addx(parameter.unwrap().parse::<i32>().unwrap()),
                "noop" => Command::Noop(),
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
    fn simple() {
        let mut commands = Vec::new();
        commands.push(Command::Noop());
        commands.push(Command::Addx(3));
        commands.push(Command::Addx(-5));
        let mut cpu = CPU::new(commands);
        let (reg, cycle) = cpu.do_cycle();
        assert_eq!(cycle, 2);
        assert_eq!(reg, 1);
        let (reg, cycle) = cpu.do_cycle();
        assert_eq!(cycle, 3);
        assert_eq!(reg, 1);
        let (reg, cycle) = cpu.do_cycle();
        assert_eq!(cycle, 4);
        assert_eq!(reg, 4);
        let (reg, cycle) = cpu.do_cycle();
        assert_eq!(cycle, 5);
        assert_eq!(reg, 4);
        let (reg, cycle) = cpu.do_cycle();
        assert_eq!(cycle, 6);
        assert_eq!(reg, -1);
    }

    #[test]
    fn complex() {
        let commands = parse_input("test_input");
        let mut cpu = CPU::new(commands);
        let mut sum = 0;
        for _ in 1..20 {
            cpu.do_cycle();
        }
        sum = sum + cpu.cycle * cpu.reg_x;
        assert_eq!(cpu.cycle, 20);
        assert_eq!(cpu.reg_x, 21);

        for _ in 0..40 {
            cpu.do_cycle();
        }
        sum = sum + cpu.cycle * cpu.reg_x;
        assert_eq!(cpu.cycle, 60);
        assert_eq!(cpu.reg_x, 19);

        for _ in 0..40 {
            cpu.do_cycle();
        }
        sum = sum + cpu.cycle * cpu.reg_x;
        assert_eq!(cpu.cycle, 100);
        assert_eq!(cpu.reg_x, 18);

        for _ in 0..40 {
            cpu.do_cycle();
        }
        sum = sum + cpu.cycle * cpu.reg_x;
        assert_eq!(cpu.cycle, 140);
        assert_eq!(cpu.reg_x, 21);

        for _ in 0..40 {
            cpu.do_cycle();
        }
        sum = sum + cpu.cycle * cpu.reg_x;
        assert_eq!(cpu.cycle, 180);
        assert_eq!(cpu.reg_x, 16);

        for _ in 0..40 {
            cpu.do_cycle();
        }
        sum = sum + cpu.cycle * cpu.reg_x;
        assert_eq!(cpu.cycle, 220);
        assert_eq!(cpu.reg_x, 18);

        assert_eq!(sum, 13140);
    }
}
