use std::fs;

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
    let mut sum: u64 = 0; 

    println!("Antinodes: {sum}");
}