use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn visible_direction(line_of_sight: &[u8], height: u8) -> bool {
    let bigger_tree = line_of_sight.iter().filter(|tree| tree >= &&height);
    return bigger_tree.count() == 0;
}

fn amount_direction(line_of_sight: &[u8], height: u8) -> usize {
    for (i, tree) in line_of_sight.iter().enumerate() {
        if tree >= &height {
            return i + 1;
        }
    }
    return line_of_sight.len();
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
    //println!("{:?}", forest);
    //println!("{:?}", columns);
    for line_index in 1..forest.len()-1 {
        for column_index in 1..forest[line_index].len()-1 {
            //println!("{}, {}", line_index, column_index);
            let height = forest[line_index][column_index];
            //println!("Height: {}", height);
            let mut visible = false;
            let west_view = &forest[line_index][..column_index];
            let east_view = &forest[line_index][column_index+1..];
            let north_view = &columns[column_index][..line_index];
            let south_view = &columns[column_index][line_index+1..];
            //println!("West View: {:?}", west_view);
            //println!("East View: {:?}", east_view);
            //println!("North View: {:?}", north_view);
            //println!("South View: {:?}", south_view);
            visible = visible || visible_direction(west_view, height);
            visible = visible || visible_direction(east_view, height);
            visible = visible || visible_direction(north_view, height);
            visible = visible || visible_direction(south_view, height);
            if visible {
                //println!("Visible");
                amount += 1;
            }
                //println!("=====================");
        }
    }
    return amount;
}

fn part_two(forest: &Vec<Trees>) -> usize {
    let mut max_score = 0;
    let columns = create_columns(forest);
    //println!("{:?}", forest);
    //println!("{:?}", columns);
    for line_index in 1..forest.len()-1 {
        for column_index in 1..forest[line_index].len()-1 {
            //println!("{}, {}", line_index, column_index);
            let height = forest[line_index][column_index];
            //println!("Height: {}", height);
            let mut west_view: Vec<u8> = forest[line_index][..column_index].iter().cloned().collect();
            west_view.reverse();
            let east_view = &forest[line_index][column_index+1..];
            let mut north_view: Vec<u8> = columns[column_index][..line_index].iter().cloned().collect();
            north_view.reverse();
            let south_view = &columns[column_index][line_index+1..];
            //println!("West View: {:?}", west_view);
            //println!("East View: {:?}", east_view);
            //println!("North View: {:?}", north_view);
            //println!("South View: {:?}", south_view);
            //visible = visible || visible_direction(west_view, height);
            let score_west = amount_direction(&west_view, height);
            let score_east = amount_direction(&east_view, height);
            let score_north = amount_direction(&north_view, height);
            let score_south = amount_direction(&south_view, height);
            let total_score = score_east * score_north * score_south * score_west;
            if total_score > max_score {
                //println!("Visible");
                max_score = total_score;
            }
                //println!("=====================");
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
        let input = parse_input().unwrap();
        let position = find_marker(&input, 4).unwrap();
        assert_eq!(1210, position);
    }

    #[test]
    fn part2_output() {
        let input = parse_input().unwrap();
        let position = find_marker(&input, 14).unwrap();
        assert_eq!(3476, position);
    }

}
