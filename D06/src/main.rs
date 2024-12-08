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
    let (map, mut x, mut y) = get_input();
    let mut direction: char = 'U';

    let mut visited: HashSet<String> = HashSet::new();

    loop {
        visited.insert(format!("{y}|{x}"));  

        let mut can_move = false;

        if direction == 'U' {
            if y == 0 {
                break;
            }
            can_move = map[y - 1][x];
        }    
        else if direction == 'D' {
            if y == map.len() -1 {
                break;
            }
            can_move = map[y + 1][x];
        }   
        else if direction == 'L' {
            if x == 0 {
                break;
            }
            can_move = map[y][x - 1];
        }   
        else if direction == 'R' {
            if x == map[0].len() -1 {
                break;
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
   
    let visited = visited.len();
    println!("Visit Count: {visited}");
}