use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn is_visible_in_direction(line_of_sight: &[u8], height: u8) -> bool {
    let bigger_tree = line_of_sight.iter().filter(|tree| tree >= &&height);
    return bigger_tree.count() == 0;
}

fn _amount_visible_direction<'a, I: Iterator<Item=&'a u8>>(line_of_sight: I, height: &'a u8) -> Option<usize> {
        for (i, tree) in line_of_sight.enumerate() {
            if tree >= &height {
                return Some(i + 1);
            }
        }
    return None;
}

fn amount_visible_direction(line_of_sight: &[u8], height: u8, reverse: bool) -> usize {
    let all = line_of_sight.len();
    if reverse {
        return _amount_visible_direction(line_of_sight.iter().rev(), &height).unwrap_or(all);
    } else {
        return _amount_visible_direction(line_of_sight.iter(), &height).unwrap_or(all);
    }
}

fn create_columns(forest: &Vec<Trees>) -> Vec<Trees> {
    let mut columns = Vec::new();
    for y in 0..forest[0].len() {
        let mut colum = Vec::new();
        for x in 0..forest.len() {
            colum.push(forest[x][y]);
        }
        columns.push(colum);
    }
    return columns;
}

fn part_one(forest: &Vec<Trees>) -> usize {
    let mut amount = forest.len() * 2 + (forest[0].len()-2) * 2;
    let columns = create_columns(forest);
    for line_index in 1..forest.len()-1 {
        for column_index in 1..forest[line_index].len()-1 {
            let height = forest[line_index][column_index];
            let mut visible = false;
            let west_view = &forest[line_index][..column_index];
            let east_view = &forest[line_index][column_index+1..];
            let north_view = &columns[column_index][..line_index];
            let south_view = &columns[column_index][line_index+1..];
            visible = visible || is_visible_in_direction(west_view, height);
            visible = visible || is_visible_in_direction(east_view, height);
            visible = visible || is_visible_in_direction(north_view, height);
            visible = visible || is_visible_in_direction(south_view, height);
            if visible {
                amount += 1;
            }
        }
    }
    return amount;
}

fn part_two(forest: &Vec<Trees>) -> usize {
    let mut max_score = 0;
    let columns = create_columns(forest);
    for line_index in 1..forest.len()-1 {
        for column_index in 1..forest[line_index].len()-1 {
            let height = forest[line_index][column_index];
            let score_west = amount_visible_direction(&forest[line_index][..column_index], height, true);
            let score_east = amount_visible_direction(&forest[line_index][column_index+1..], height, false);
            let score_north = amount_visible_direction(&columns[column_index][..line_index], height, true);
            let score_south = amount_visible_direction(&columns[column_index][line_index+1..], height, false);
            let total_score = score_east * score_north * score_south * score_west;
            if total_score > max_score {
                max_score = total_score;
            }
        }
    }
    return max_score;
}

fn main() {
    let forest = parse_input();
    let sol_part_one = part_one(&forest);
    println!{"Solution part 1: {}", sol_part_one};
    let sol_part_two = part_two(&forest);
    println!{"Solution part 2: {}", sol_part_two};
}


type Trees = Vec<u8>;

fn parse_input() -> Vec<Trees> {
    let mut result = Vec::new();
    for line in read_lines("input").unwrap() {
        if let Ok(linedata) = line {
            let mut treeline = Vec::new();
            for tree in linedata.chars() {
                let tree = tree.to_string().parse::<u8>().unwrap();
                treeline.push(tree);
            }
            result.push(treeline);
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
    fn part1_output() {
        let input = parse_input();
        let amount_visible = part_one(&input);
        assert_eq!(1851, amount_visible);
    }

    #[test]
    fn part2_output() {
        let input = parse_input();
        let score = part_two(&input);
        assert_eq!(574080, score);
    }

}
