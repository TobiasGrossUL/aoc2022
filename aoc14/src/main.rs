use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp;

fn main() {
    part_one();
    part_two();
}

fn max(input: &Vec<Pointline>) -> (usize, usize) {
    let mut max_x = 0;
    let mut max_y = 0;
    for line in input {
        for point in line {
            max_x = cmp::max(max_x, point.0);
            max_y = cmp::max(max_y, point.1);
        }
    }
    return (max_x, max_y);
}

fn part_test() -> usize {
    let mut input = parse_input("test_input");
    let (width, height) = max(&input);
    println!("Width: {}, Height: {}", width, height);
    let mut sim = MaterialSimulation::new(width + 30, height + 3);
    input.push(vec![(0, height + 2),(width +  29, height + 2)]);
    sim.init_material(&input);
    let result = sim.simulate();
    sim.draw();
    println!("Solution part 1: {}", result);
    return result;
}

fn part_one() -> usize {
    let input = parse_input("input");
    let (width, height) = max(&input);
    let mut sim = MaterialSimulation::new(width + 1, height + 1);
    sim.init_material(&input);
    let result = sim.simulate();
    println!("Solution part 1: {}", result);
    return result;
}

fn part_two() -> usize {
    let mut input = parse_input("input");
    let (width, height) = max(&input);
    let width = width * 2;
    let height = height +3;
    let mut sim = MaterialSimulation::new(width, height);
    input.push(vec![(0, height -1),(width -1, height - 1)]);
    sim.init_material(&input);
    let result = sim.simulate();
    sim.draw();
    println!("Solution part 1: {}", result);
    return result;
}

#[derive(Clone, PartialEq)]
enum Material {
    Stone,
    Air,
    Sand,
}

struct MaterialSimulation {
    material: Vec<Vec<Material>>,
    start: Point,
    void: usize,
}

impl MaterialSimulation {
    fn new(width: usize, height: usize) -> Self {
        let material = vec![vec![Material::Air; height]; width];
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
                self.material[x][y] = Material::Stone;
            }
        } else {
            let y = a.1;
            let from = cmp::min(a.0, b.0);
            let to = cmp::max(a.0, b.0);
            for x in from..=to {
                self.material[x][y] = Material::Stone;
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
        return self.material[place.0][place.1 + 1] == Material::Air;
    }

    fn is_down_left_free(&self, place: &Point) -> bool {
        return self.material[place.0-1][place.1 + 1] == Material::Air;
    }

    fn is_down_right_free(&self, place: &Point) -> bool {
        return self.material[place.0+1][place.1 + 1] == Material::Air;
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
            self.material[place.0][place.1] = Material::Sand;
            return false;
        }

        //nothing free => manifest_sand
        self.material[place.0][place.1] = Material::Sand;

        return true;

    }

    fn draw(&self) {
        let min_x = self.material.len() - 60;
        let max_x = self.material.len();
        for y in 0..self.material[0].len() {
            for x in min_x..max_x {
                match self.material[x][y] {
                    Material::Air => print!("."),
                    Material::Stone => print!("#"),
                    Material::Sand => print!("o"),
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
        assert_eq!(solution, 6240);
    }

    #[test]
    fn part_two_test() {
        let solution = part_two();
        assert_eq!(solution, 23142);
    }

    #[test]
    fn compare_packet() {
        let p1 = Packet::Item(1);
        let p2 = Packet::Item(1);
        let result = p1 == p2;
        assert_eq!(result, true);
    }
}
