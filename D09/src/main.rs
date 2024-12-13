use std::fs;

fn get_input() -> Vec<usize> {

    let result: Vec<Vec<usize>> = fs::read_to_string("./D09/input.txt")
        .expect("File exists.").replace("\r\n", "\n")
        .split("\n")
        .map(|x| x.chars().map(|y| y as usize - 48).collect()) //48 = reliant on UTF8 and input being numbers
        .collect();

    result[0].clone()
}

fn main() {
    let input = get_input();      
    let mut compacted: Vec<Option<usize>> = Vec::new();
    let mut is_file = true;
    let mut file_id:usize = 0;
    let mut sum:usize = 0;

    for item in input {
        if is_file {
            for _ in 0..item
            {
                compacted.push(Option::from(file_id));
            }

            file_id += 1;
        }
        else {
            for _ in 0..item
            {
                compacted.push(None);
            }
        }
        is_file = !is_file;
    }

    let mut swap_index = compacted.len() -1;
    for i in 0..compacted.len() {
        if compacted[i] == None
        {
            while compacted[swap_index] == None {
                swap_index -= 1;
            }

            if swap_index <= i {
                break;
            }

            compacted.swap(i, swap_index);
            swap_index -= 1;
        }

        if compacted[i] != None {
            sum += i * compacted[i].unwrap();
        }
    }

    println!("Output 1 {sum}");
}