use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Map {
    start: (usize, usize),
    end: (usize, usize),
    grid: Vec<Vec<u32>>,
    visited: HashMap<(usize, usize), usize>,
    finish_on_a: bool
}

impl Map {
    fn new(start: (usize, usize), end: (usize, usize), grid: Vec<Vec<u32>>) -> Map {
        let visited = HashMap::new();
        Map {start, end, grid, visited, finish_on_a: false}
    }

    fn _reverse_steps(&self, current: &(usize, usize)) -> Vec<(usize, usize)> {
        let mut result = Vec::new();

        let min_elevation = self.grid[current.0][current.1] as i64 - 1;
        let min_elevation = if min_elevation >= 0 {
            min_elevation as u32
        } else {
            0
        };

        // four directions
        if current.0 > 0 {
            result.push((current.0 -1, current.1));
        }
        if current.1 > 0 {
            result.push((current.0, current.1 -1));
        }

        if current.0 < self.grid.len() - 1 {
            result.push((current.0 +1, current.1));
        }

        if current.1 < self.grid[0].len() - 1 {
            result.push((current.0, current.1 +1));
        }

        // filter only legit targets
        result = result.iter().filter(|x| self.grid[x.0][x.1] >= min_elevation).copied().collect();
        return result;
    }

    fn _check_target_reached(&self, position: &(usize, usize)) -> bool {
        if self.finish_on_a {
            return self.grid[position.0][position.1] == 0;
        } else {
            return position == &self.start;
        }
    }

    fn _check_allready_visited(&mut self, position: &(usize, usize), step: usize) -> bool {
        if self.visited.contains_key(&position) && self.visited.get(&position).unwrap() <= &step {
                return true;
        } else {
            self.visited.insert(position.clone(), step);
            return false;
        }
    }

    // returns true if target is reached and amount of steps required
    fn _walk_reverse(&mut self, steps: usize, position: &(usize, usize)) -> (bool, usize) {
        // check if we hit target
        if self._check_target_reached(position) {
            return (true, steps);
        }

        if self._check_allready_visited(position, steps) {
            return (false, 0);
        }

        let next_positions = self._reverse_steps(position);

        let mut min_steps = std::usize::MAX;
        for next_pos in next_positions {
            let (reached, variant_steps) = self._walk_reverse(steps + 1, &next_pos);
            if reached  && variant_steps < min_steps {
                min_steps =  variant_steps;
            }
        }

        if min_steps < std::usize::MAX {
            return (true, min_steps);
        } else {
            return (false,0);
        }
    }

    fn start_walking(&mut self) -> usize {
        return self._walk_reverse(0, &self.end.clone()).1;
    }
}

fn main() {
    part_one();
    part_two();
}

fn part_one() -> usize {
    let mut map = parse_input("input");
    let result = map.start_walking();
    println!("Solution part1: {}", result);
    return result;
}

fn part_two() -> usize {
    let mut map = parse_input("input");
    map.finish_on_a = true;
    let result = map.start_walking();
    println!("Solution part2: {}", result);
    return result;
}

fn parse_input(filename: &str) -> Map {
    let mut result = Vec::new();
    let mut start_index = (0, 0);
    let mut target_index = (0, 0);
    for (row_index, line) in read_lines(filename).unwrap().enumerate() {
        if let Ok(linedata) = line {
            let mut row = Vec::new();
            for (column_index, element) in linedata.chars().enumerate() {
                let elevation = match element {
                    'S' => {start_index = (row_index, column_index); 0},
                    'E' => {target_index = (row_index, column_index); 'z' as u32 - 'a' as u32},
                    _ => {element as u32 - 'a' as u32},
                };
                row.push(elevation);
            }
            result.push(row);
        }
    }
    return Map::new(start_index, target_index, result);
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
    fn test_input() {
        let mut map = parse_input("test_input");
        let result = map.start_walking();
        assert_eq!(result, 31);
    }

    #[test]
    fn test_tuple_equal() {
        let map = parse_input("test_input");
        let result = map.start == (0,0);
        assert_eq!(result, true);

        let result = map.end == (2,5);
        assert_eq!(result, true);
    }


    #[test]
    fn part_one_test() {
        let solution = part_one();
        assert_eq!(solution, 517);
    }

    #[test]
    fn part_two_test() {
        let solution = part_two();
        assert_eq!(solution, 512);
    }
}
