use std::fs;
// use std::collections::HashMap;

fn get_input() -> Vec<Vec<char>> {
    let contents = fs::read_to_string("./D04/input.txt")
        .expect("File exists.");

    return contents.replace("\r\n", "\n")
        .split("\n")
        .map(|i|String::from(i).chars().collect())
        .collect();
}


fn main() {
    let input = get_input();
    let mut count:u64 = 0;
    let mut mas_count:u64 = 0;
     
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if input[y][x] == 'X' {
                //Up Left
                if y >= 3 && x >= 3
                    && input[y-1][x-1] == 'M'
                    && input[y-2][x-2] == 'A'
                    && input[y-3][x-3] == 'S' {
                    count += 1;
                }
                //Up
                if y >= 3
                    && input[y-1][x] == 'M'
                    && input[y-2][x] == 'A'
                    && input[y-3][x] == 'S' {
                    count += 1;
                }
                //Up Right
                if y >= 3 && x < input[y].len() - 3
                    && input[y-1][x+1] == 'M'
                    && input[y-2][x+2] == 'A'
                    && input[y-3][x+3] == 'S' {
                    count += 1;
                }
                //Right
                if x < input[y].len() - 3
                    && input[y][x+1] == 'M'
                    && input[y][x+2] == 'A'
                    && input[y][x+3] == 'S' {
                    count += 1;
                }
                //Down Right
                if y < input.len() - 3 && x < input[y].len() - 3
                    && input[y+1][x+1] == 'M'
                    && input[y+2][x+2] == 'A'
                    && input[y+3][x+3] == 'S' {
                    count += 1;
                }
                //Down
                if y < input.len() - 3
                    && input[y+1][x] == 'M'
                    && input[y+2][x] == 'A'
                    && input[y+3][x] == 'S' {
                    count += 1;
                }
                //Down Left
                if y < input.len() - 3 && x >= 3
                    && input[y+1][x-1] == 'M'
                    && input[y+2][x-2] == 'A'
                    && input[y+3][x-3] == 'S' {
                    count += 1;
                }
                //Left
                if x >= 3
                    && input[y][x-1] == 'M'
                    && input[y][x-2] == 'A'
                    && input[y][x-3] == 'S' {
                    count += 1;
                }
            }

            if input[y][x] == 'A' 
                && x > 0 && x < input[y].len() - 1
                && y > 0 && y < input.len() - 1 {
                
                //left - right
                if (input[y-1][x-1] == 'M' && input[y+1][x+1] == 'S')
                || (input[y-1][x-1] == 'S' && input[y+1][x+1] == 'M') {
                    // right - left
                    if (input[y-1][x+1] == 'M' && input[y+1][x-1] == 'S')
                    || (input[y-1][x+1] == 'S' && input[y+1][x-1] == 'M') {
                        mas_count += 1;
                    }
                }
            }
        }
    }

    println!("XMAS Count: {count}");
    println!("X-MAS Count: {mas_count}");
}