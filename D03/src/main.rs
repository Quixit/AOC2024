use std::fs;
// use std::collections::HashMap;

fn get_input() -> String {
    let contents = fs::read_to_string("./D03/input.txt")
        .expect("File exists.");

    return contents;
}

fn main() {
    let input: Vec<char> = get_input().chars().collect();
    let mut sum:u64 = 0;
    let mut activesum:u64 = 0;
    let mut active = true;

    for mut i in 0..input.len() {
        if input[i] == 'm' && i < input.len() - 4 {
            let slice = &input[i..i+4];
            let slice: String = slice.into_iter().collect();

            if slice.as_str() == "mul("
            {
                i += 4;

                let mut op1 = String::new();
                let mut op2 = String::new();

                for x in i..input.len()
                {
                    match input[x] {
                        '0'..':' => {
                            op1.push(input[x]);
                        },
                        ',' => {
                            i = x + 1;
                            break;
                        },
                        _ => {
                            op1.clear();
                            break;
                        }
                    }
                }

                if op1.len() > 0 {
                
                    for x in i..input.len()
                    {
                        match input[x] {
                            '0'..':' => {
                                op2.push(input[x]);
                            },
                            ')' => {
                                //i = x + 1;
                                break;
                            },
                            _ => {
                                op2.clear();
                                break;
                            }
                        }
                    }

                    if op2.len() > 0 {
                        let one: u64 = op1.parse().unwrap();
                        let two: u64 = op2.parse().unwrap();

                        sum += one * two;

                        if active {
                            activesum += one * two;
                        }
                    }
                }
            }
        }

        if input[i] == 'd' && i < input.len() - 4 {
            let do_slice = &input[i..i+4];
            let do_slice: String = do_slice.into_iter().collect();

            if do_slice.as_str() == "do()"
            {
                active = true;
            }

            let dont_slice = &input[i..i+7];
            let dont_slice: String = dont_slice.into_iter().collect();

            if dont_slice.as_str() == "don't()"
            {
                active = false;
            }
        }
    }

    println!("Result: {sum}");
    println!("Active Result: {activesum}");
}