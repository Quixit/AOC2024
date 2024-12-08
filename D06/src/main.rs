use std::{collections::HashSet, fs};

fn get_input() -> (Vec<Vec<bool>>, usize, usize) {
    let mut result: Vec<Vec<bool>> = Vec::new(); 
    let mut result_x: usize = 0; 
    let mut result_y: usize = 0;

    let mut player = |x: usize, y: usize| -> bool {
        result_x = x;
        result_y = y;

        true
    };

    let contents: Vec<String> = fs::read_to_string("./D06/input.txt")
        .expect("File exists.").replace("\r\n", "\n")
        .split("\n")
        .map(|x| x.to_string())
        .collect();

    for y in 0..contents.len()
    {
        let line: Vec<char> = contents[y].chars().collect();
        let mut array:Vec<bool> = Vec::new();

        for x in 0..line.len()  {
            let value = match line[x] {
                '#' => false,
                '^' => player(x, y),
                _ => true    
            };
            array.push(value);
        }

        result.push(array);
    }

    return  (result, result_x, result_y);
}


fn main() {
    let (map, x_in, y_in) = get_input();      
    let (cycle, visited) = simulate(&map, &x_in, &y_in);

    if cycle {
        panic!("Invalid, can't have cycles in initial");
    }
    else {
        let visit_count = visited.len();
        println!("Visit Count: {visit_count}");
    }

    let mut cycle_count: u64 = 0;

    for visit in visited {
        let mut new_map = map.clone();
        let (y, x) = visit;

        new_map[y][x] = false;

        let (cycle, _) = simulate(&new_map, &x_in, &y_in);

        if cycle {
            cycle_count += 1;
        }
    }

    println!("Cycle Count: {cycle_count}");

}

fn simulate(map: &Vec<Vec<bool>>, x_in: &usize, y_in: &usize) -> (bool, HashSet<(usize, usize)>) {
    let mut x = x_in.clone();
    let mut y = y_in.clone();
    let mut direction: char = 'U';

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut history: HashSet<String> = HashSet::new();

    loop {
        let record = format!("{y}|{x}|{direction}");

        if history.contains(&record) {
            return (true, visited);
        }
        else {
             visited.insert((y,x));  
             history.insert(record);
        }       

        let mut can_move = false;

        if direction == 'U' {
            if y == 0 {
                return (false, visited);
            }
            can_move = map[y - 1][x];
        }    
        else if direction == 'D' {
            if y == map.len() -1 {
                return (false, visited);
            }
            can_move = map[y + 1][x];
        }   
        else if direction == 'L' {
            if x == 0 {
                return (false, visited);
            }
            can_move = map[y][x - 1];
        }   
        else if direction == 'R' {
            if x == map[0].len() -1 {
                return (false, visited);
            }
            can_move = map[y][x + 1];
        }

        if can_move {
            match direction {
                'U' => y -= 1,
                'D' => y += 1,
                'L' => x -= 1,
                'R' => x += 1,
                _ => panic!("invalid direction")
            }                
        }
        else {
            direction = match direction {
                'U' => 'R',
                'D' => 'L',
                'L' => 'U',
                'R' => 'D',
                _ => panic!("invalid direction")
            }
        }        
    }
}