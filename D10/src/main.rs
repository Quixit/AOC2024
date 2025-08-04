use std::fs;
use crate::results::Results;

mod results;

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
    let mut sum_two = 0;

    for y in 0..input.len() {
        for x in 0..input[0].len() {
            let score = find_score(y, x, &input);

            sum_one += score.peaks.len();
            sum_two += score.trails.len();
        }
    }

    println!("Output 1 {sum_one}");
    println!("Output 2 {sum_two}");
}

fn find_score(y: usize, x: usize, input: &Vec<Vec<u8>>) -> Results {
    let height = input[y][x];

    let mut result = Results::new();

    if height != 0 {
        return result;
    }

    find_peaks(y, x, &mut result, Vec::new(), input);

    result
}

fn find_peaks(y: usize, x: usize, result: &mut Results, mut trail: Vec<(usize, usize)>, input: &Vec<Vec<u8>>) {
    let height = input[y][x];

    trail.push((x,y));

    if height == 9 {
        result.peaks.insert((y, x));
        result.trails.insert(trail);

        return;
    }

    if y+1 < input.len() && input[y+1][x] != 0 && input[y+1][x] - 1 == height {
        find_peaks(y+1, x, result, trail.clone(), &input);
    }

    if y >= 1 && input[y-1][x] != 0 && input[y-1][x] - 1 == height {
        find_peaks(y-1, x, result, trail.clone(), &input);
    }

    if x+1 < input[0].len() && input[y][x+1] != 0 && input[y][x+1] - 1  == height {
        find_peaks(y, x+1, result, trail.clone(), &input);
    }

    if x >= 1 && input[y][x-1] != 0  && input[y][x-1] - 1 == height {
        find_peaks(y, x-1, result, trail.clone(), &input);
    }
}