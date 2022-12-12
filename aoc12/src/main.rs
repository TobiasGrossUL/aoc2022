use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Map {
    start: (usize, usize),
    end: (usize, usize),
    grid: Vec<Vec<u32>>,
    max_steps: usize,
    visited: HashMap<(usize, usize), usize>
}

impl Map {
    fn new(start: (usize, usize), end: (usize, usize), grid: Vec<Vec<u32>>) -> Map {
        let max_steps = grid.len() * grid[0].len();
        let visited = HashMap::new();
        Map {start, end, grid, max_steps, visited}
    }

    fn _possible_steps(&self, current: (usize, usize)) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        let max_elevation = self.grid[current.0][current.1] + 1;
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

        return result.iter().filter(|x| self.grid[x.0][x.1] <= max_elevation).copied().collect();
    }

    // returns true if target is reached and amount of steps required
    fn _walk(&mut self, steps: usize, position: (usize, usize)) -> (bool, usize) {
        // check if we hit target
        if position == self.end {
            return (true, steps);
        }

        // check if we exceed max steps
        if steps >= self.max_steps {
            return (false, 0);
        }

        if self.visited.contains_key(&position){
            if self.visited.get(&position).unwrap() > &steps {
                self.visited.insert(position, steps);
            } else {
                return (false, 0);
            }
        } else {
            self.visited.insert(position, steps);
        }

        let next_positions = self._possible_steps(position);

        let mut steps_to_finish = None;
        for next_pos in next_positions {
            let (reached, variant_steps) = self._walk(steps + 1, next_pos);
            if reached {
                if steps_to_finish.is_none() {
                    steps_to_finish = Some(variant_steps);
                } else {
                    if steps_to_finish.unwrap() > variant_steps {
                        steps_to_finish = Some(variant_steps);
                    }
                }
            }
        }

        if steps_to_finish.is_some() {
            return (true, steps_to_finish.unwrap());
        } else {
            return (false,0);
        }
    }

    fn _best_reverse_steps(&self, current: &(usize, usize)) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        let min_elevation;
        if self.grid[current.0][current.1] == 0 {
            min_elevation = self.grid[current.0][current.1];
        } else {
            min_elevation = self.grid[current.0][current.1] - 1;
        }

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

        result = result.iter().filter(|x| {
            self.grid[x.0][x.1] >= min_elevation
        }).copied().collect();
        return result;
    }

    // returns true if target is reached and amount of steps required
    fn _walk_reverse(&mut self, steps: usize, position: &(usize, usize), max_steps: usize) -> (bool, usize) {
        let mut local_max_steps = max_steps;
        // check if we hit target
        if position == &self.start {
            return (true, steps);
        }

        if position == &self.start {
            return (true, steps);
        }

        // check if we exceed max steps
        if steps >= max_steps {
            return (false, 0);
        }

        if self.visited.contains_key(&position){
            if self.visited.get(&position).unwrap() > &steps {
                self.visited.insert(position.clone(), steps);
            } else {
                return (false, 0);
            }
        } else {
            self.visited.insert(position.clone(), steps);
        }

        let next_positions = self._best_reverse_steps(position);

        let mut steps_to_finish = None;
        for next_pos in next_positions {
            let (reached, variant_steps) = self._walk_reverse(steps + 1, &next_pos, local_max_steps);
            if reached {
                local_max_steps = variant_steps;
                if steps_to_finish.is_none() {
                    steps_to_finish = Some(variant_steps);
                } else {
                    if steps_to_finish.unwrap() > variant_steps {
                        steps_to_finish = Some(variant_steps);
                    }
                }
            }
        }

        if steps_to_finish.is_some() {
            return (true, steps_to_finish.unwrap());
        } else {
            return (false,0);
        }
    }

    fn start_walking(&mut self) -> usize {
        //return self._walk(0, self.start).1;
        return self._walk_reverse(0, &self.end.clone(), self.max_steps).1;
    }
}

fn main() {
    test();
    part_one();
    part_two();
}

fn test() -> usize {
    let mut map = parse_input("test_input");
    println!("Max Steps: {}", map.max_steps);
    let result = map.start_walking();
    println!("Solution test: {}", result);
    return result;
}

fn part_one() -> usize {
    let mut map = parse_input("input");
    println!("Max Steps: {}", map.max_steps);
    let result = map.start_walking();
    println!("Solution part1: {}", result);
    return result;
}

fn part_two() -> i64{
    return 0;
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
    fn test_possible_steps() {
        let map = parse_input("test_input");
        let psteps = map._possible_steps((0,0));
        assert_eq!(psteps.len(), 2);

        let ele = map.grid[1][2];
        assert_eq!(ele, 2);

        let psteps = map._possible_steps((1,2));
        assert_eq!(psteps.len(), 3);

        let psteps = map._possible_steps((2,5));
        assert_eq!(psteps.len(), 4);
    }

    #[test]
    fn test_input() {
        let mut map = parse_input("test_input");
        map.max_steps = 35;
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
        //let solution = part_one();
        //assert_eq!(solution, 55216);
    }

    #[test]
    fn part_two_test() {
        //let solution = part_two();
        //assert_eq!(solution, 12848882750);
    }
}
