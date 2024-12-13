use std::{collections::{HashMap, HashSet}, fs, usize};

fn get_input() -> Vec<Vec<char>> {

    let result: Vec<Vec<char>> = fs::read_to_string("./D08/input.txt")
        .expect("File exists.").replace("\r\n", "\n")
        .split("\n")
        .map(|x| x.chars().collect())
        .collect();

    return  result;
}

fn main() {
    let input = get_input();      
    let mut nodes: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    let mut antinodes: HashSet<(isize, isize)> = HashSet::new(); 

    let y_max: isize = input.len().try_into().unwrap();
    let x_max: isize = input[0].len().try_into().unwrap();

    for y in 0..y_max {
        for x in 0..x_max {
            let yu: usize  = y.try_into().unwrap();
            let xu: usize  = x.try_into().unwrap();

            let target: char = input[yu][xu];

            if target != '.' {
                let mut node = match nodes.get(&target) {
                    None => Vec::new(),
                    x => x.unwrap().clone()
                };

                node.push((y, x));
                
                nodes.insert(target, node);
            }
        }
    }

    for node in nodes {
        for point_one in node.1.clone() {
            for point_two in node.1.clone() {
                if point_one != point_two
                {
                    let found = get_node(point_one, point_two, y_max, x_max);

                    if found != None {
                        let found = found.unwrap();

                        antinodes.insert(found); 
                    }
                }
            }
        }
    }

    let node_count = antinodes.len();
    println!("Antinodes: {node_count}");
}

fn get_node (from:(isize, isize), to: (isize, isize), y_max:isize, x_max:isize) -> Option<(isize, isize)> {
    let y_result = to.0 + (to.0 - from.0);
    let x_result = to.1 + (to.1 - from.1);

    if y_result < 0 || y_result > (y_max -1) || x_result < 0 || x_result > (x_max -1) {
        return None;
    }

    return Some((
        y_result,
        x_result
    )); 
}