use std::fs;

fn get_input() -> Vec<u64> {

    let result: Vec<u64> = fs::read_to_string("./D11/input.txt")
        .expect("File exists.")
        .split(" ")
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    return  result;
}

fn main() {
    let input = get_input();      
    let mut state_one = input.clone();
    //let mut state_two = input.clone();
    const BLINKS: usize = 6;

    for _ in 0..BLINKS {

        for mut x in 0..input.len() {
            match state_one[x] {
                0 => state_one[x] = 1,
                even if even % 2 == 0 => {
                    let string_value = state_one[x].to_string();

                    let one: String = string_value.chars().take(string_value.len() / 2).collect();
                    let two: String = string_value.chars().skip(string_value.len() / 2).collect();

                    state_one[x] = one.parse::<u64>().unwrap();
                    state_one.insert(x + 1, two.parse::<u64>().unwrap());

                    x += 1;
                },
                _ => state_one[x] = state_one[x] * 2024
            }
        }
    }

    let length_one = state_one.len();
    //let length_two = state_two.len();
    println!("Output 1 {length_one}");
    //println!("Output 2 {length_two}");
}