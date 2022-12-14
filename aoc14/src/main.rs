use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp;

fn main() {
    part_one();
    part_two();
}

fn max_height(input: &Vec<Pointline>) -> usize {
    let mut max_y = 0;
    for line in input {
        for point in line {
            max_y = cmp::max(max_y, point.1);
        }
    }
    return max_y + 1;
}

fn part_one() -> usize {
    let input = parse_input("input");
    let height = max_height(&input);
    let mut sim = MaterialSimulation::new(height + 1);
    sim.init_material(&input);
    let result = sim.simulate();
    println!("Solution part 1: {}", result);
    return result;
}

fn part_two() -> usize {
    let mut input = parse_input("input");
    let height = max_height(&input);
    let height = height + 1;
    let mut sim = MaterialSimulation::new(height + 1);
    input.push(vec![(300, height),(700, height)]);
    sim.init_material(&input);
    let result = sim.simulate() + 1;
    println!("Solution part 2: {}", result);
    return result;
}

#[derive(Clone, PartialEq)]
enum Material {
    Stone,
    Sand,
}

struct MaterialSimulation {
    material: HashMap<Point, Material>,
    start: Point,
    void: usize,
}

impl MaterialSimulation {
    fn new(height: usize) -> Self {
        let material = HashMap::new();
        let start = (500, 0);
        let void = height;
        Self {material, start, void}
    }

    fn draw_stone(&mut self, a: &Point, b: &Point)  {
        if a.0 == b.0 {
            let x = a.0;
            let from = cmp::min(a.1, b.1);
            let to = cmp::max(a.1, b.1);
            for y in from..=to {
                self.material.insert((x, y), Material::Stone);
            }
        } else {
            let y = a.1;
            let from = cmp::min(a.0, b.0);
            let to = cmp::max(a.0, b.0);
            for x in from..=to {
                self.material.insert((x, y), Material::Stone);
            }
        }
    }

    fn init_material(&mut self, pointlines: &Vec<Pointline>) {
        for line in pointlines {
            for endpoints in line.windows(2) {
                self.draw_stone(&endpoints[0], &endpoints[1]);
            }
        }
    }

    fn is_down_free(&self, place: &Point) -> bool {
        return ! self.material.contains_key(&(place.0, place.1 + 1));
    }

    fn is_down_left_free(&self, place: &Point) -> bool {
        return ! self.material.contains_key(&(place.0 - 1, place.1 + 1));
    }

    fn is_down_right_free(&self, place: &Point) -> bool {
        return ! self.material.contains_key(&(place.0 + 1, place.1 + 1));
    }

    fn drop_sand(&mut self, place: Point) -> bool {
        //check if we are allready falling into void
        if place.1 >= self.void - 1 {
            return false;
        }

        if self.is_down_free(&place) {
            return self.drop_sand((place.0, place.1 + 1));
        } else if self.is_down_left_free(&place) {
            return self.drop_sand((place.0-1, place.1 + 1));
        } else if self.is_down_right_free(&place) {
            return self.drop_sand((place.0+1, place.1 + 1));
        }

        //nothing free and we reached source
        if place == self.start {
            self.material.insert(place, Material::Sand);
            return false;
        }

        //nothing free => manifest_sand
        self.material.insert(place, Material::Sand);

        return true;

    }

    fn draw(&self, topleft: Point, bottomright: Point) {
        for y in topleft.1..bottomright.1 {
            for x in topleft.0..bottomright.0 {
                if self.material.contains_key(&(x,y)) {
                    match self.material[&(x, y)] {
                        Material::Stone => print!("#"),
                        Material::Sand => print!("o"),
                    }
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }

    fn simulate(&mut self) -> usize {
        let mut counter = 0;
        while self.drop_sand(self.start) {
            counter += 1;
        }
        return counter;
    }
}

type Point = (usize, usize);
type Pointline = Vec<Point>;

fn parse_input(filename: &str) -> Vec<Pointline> {
    let mut res = Vec::new();
    for line in read_lines(filename).unwrap() {
        if let Ok(linedata) = line {
            let mut linepoints = Vec::new();
            let points = linedata.split(" -> ");
            for point in points {
                let mut xy = point.split(",");
                let x = xy.nth(0).unwrap().parse::<usize>().unwrap();
                let y = xy.nth(0).unwrap().parse::<usize>().unwrap();
                let point = (x, y);
                linepoints.push(point);
            }
            res.push(linepoints);
        }
    }
    return res;
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
        assert_eq!(solution, 888);
    }

    #[test]
    fn part_two_test() {
        let solution = part_two();
        assert_eq!(solution, 26461);
    }
}
