use std::{collections::HashSet, fs};

fn get_input() -> Vec<Vec<u8>> {

    let result: Vec<Vec<u8>> = fs::read_to_string("./D10/input.txt")
        .expect("File exists.").replace("\r\n", "\n")
        .split("\n")
        .map(|x| x.chars().map(|y| y as u8 - 48).collect())
        .collect();

    return  result;
}

fn main() {
    let input = get_input();      
    let mut sum_one = 0;
    //let mut sum_two = 0;

    for y in 0..input.len() {
        for x in 0..input[0].len() {
            sum_one += find_score(y, x, &input).len();
        }
    }

    println!("Output 1 {sum_one}");
    //println!("Output 2 {sum_two}");
}

fn find_score(y: usize, x: usize, input: &Vec<Vec<u8>>) -> HashSet<(usize, usize)> {
    let height = input[y][x];

    let mut result: HashSet<(usize, usize)> = HashSet::new();

    if y+1 < input.len() && input[y+1][x] != 0 && input[y+1][x] - 1 == height {
        let down = find_score(y+1, x, &input);

        for i in down {
            result.insert(i);
        }
    }

    if y >= 1 && input[y-1][x] != 0 && input[y-1][x] - 1 == height {
        let up = find_score(y-1, x, &input);
        
        for i in up {
            result.insert(i);
        }
    }

    if x+1 < input[0].len() && input[y][x+1] != 0 && input[y][x+1] - 1  == height {
        let right = find_score(y, x+1, &input);
        
        for i in right {
            result.insert(i);
        }
    }

    if x >= 1 && input[y][x-1] != 0  && input[y][x-1] - 1 == height {
        let left = find_score(y, x-1, &input);
        
        for i in left {
            result.insert(i);
        }
    }

    result
}