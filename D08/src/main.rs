use std::{collections::HashSet, fs, usize};

fn get_input() -> Vec<Vec<char>> {

    let result: Vec<Vec<char>> = fs::read_to_string("./D08/input.txt")
        .expect("File exists.").replace("\r\n", "\n")
        .split("\n")
        .map(|x| x.chars().collect())
        .collect();

    return  result;
}

enum Movment  {
    Plus,
    Minus,
    None
}


fn main() {
    let input = get_input();      
    let mut nodes: HashSet<(isize, isize)> = HashSet::new(); 

    let y_max: isize = input.len().try_into().unwrap();
    let x_max: isize = input[0].len().try_into().unwrap();

    for y in 0..y_max {
        for x in 0..x_max {
            let yu: usize  = y.try_into().unwrap();
            let xu: usize  = x.try_into().unwrap();

            let target: char = input[yu][xu];

            if target == '.' {
                continue;
            }

            //Up Left
            let node = get_node( 
                navigate_until_found(target, &input, y, x, y_max, x_max, (Movment::Minus, Movment::Minus)),
                y,
                x, y_max, x_max
            );

            if node != None {
                nodes.insert(node.unwrap());
            }

            //Up
            let node = get_node( 
                navigate_until_found(target, &input, y, x, y_max, x_max, (Movment::Minus, Movment::None)),
                y,
                x, y_max, x_max
            );

            if node != None {
                nodes.insert(node.unwrap());
            }


            //Up Right
            let node = get_node( 
                navigate_until_found(target, &input, y, x, y_max, x_max, (Movment::Minus, Movment::Plus)),
                y,
                x, y_max, x_max
            );

            if node != None {
                nodes.insert(node.unwrap());
            }


            //Right
            let node = get_node( 
                navigate_until_found(target, &input, y, x, y_max, x_max, (Movment::None, Movment::Plus)),
                y,
                x, y_max, x_max
            );

            if node != None {
                nodes.insert(node.unwrap());
            }


            //Down Right
            let node = get_node( 
                navigate_until_found(target, &input, y, x, y_max, x_max, (Movment::Plus, Movment::Plus)),
                y,
                x, y_max, x_max
            );

            if node != None {
                nodes.insert(node.unwrap());
            }


            //Down
            let node = get_node( 
                navigate_until_found(target, &input, y, x, y_max, x_max, (Movment::Plus, Movment::None)),
                y,
                x, y_max, x_max
            );

            if node != None {
                nodes.insert(node.unwrap());
            }


            //Down left
            let node = get_node( 
                navigate_until_found(target, &input, y, x, y_max, x_max, (Movment::Plus, Movment::Minus)),
                y,
                x, y_max, x_max
            );

            if node != None {
                nodes.insert(node.unwrap());
            }


            //Left
            let node = get_node( 
                navigate_until_found(target, &input, y, x, y_max, x_max, (Movment::None, Movment::Minus)),
                y,
                x, y_max, x_max
            );

            if node != None {
                nodes.insert(node.unwrap());
            }

        }
    }

    let node_count = nodes.len();
    println!("Antinodes: {node_count}");
}

fn navigate_until_found(target: char, map: &Vec<Vec<char>>, mut y: isize, mut x:isize, y_max:isize, x_max:isize, transform: (Movment, Movment)) -> Option<(isize, isize)> {
    
    match transform.0 {
        Movment::Plus => {
            y += 1;
        }
        Movment::Minus => {
            y -= 1;
        }
        _ => {}
    }

    match transform.1 {
        Movment::Plus => {
            x += 1;
        }
        Movment::Minus => {
            x -= 1;
        }
        _ => {}
    }

    if y < 0 || y >= y_max || x < 0 || x >= x_max {
        return  None;
    }

    let yu: usize  = y.try_into().unwrap();
    let xu: usize  = x.try_into().unwrap();

    println!("{yu}-{xu}");

    if map[yu][xu] == target {
        return Some((y, x));
    } else {
        return navigate_until_found(target, map, y, x, y_max, x_max, transform);
    }    
}

fn get_node (found: Option<(isize, isize)>, y:isize, x:isize, y_max:isize, x_max:isize) -> Option<(isize, isize)> {
    if found != None {
        let unwrapped = found.unwrap();

        let y_result = y - (y - unwrapped.0);
        let x_result = x - (x - unwrapped.1);

        if y_result < 0 || y_result > (y_max -1) || x_result < 0 || x_result > (x_max -1) {
            return None;
        }

        return Some((
            y_result,
            x_result
        )); 
    }
    else {
        return None;
    }
}