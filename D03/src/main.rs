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

    for mut i in 0..input.len() {
        if input[i] == 'm' && i < input.len() - 4 {
            let slice = &input[i..i+4];
            let slice: String = slice.into_iter().collect();

            let operation = match slice.as_str() {
                "mul(" => Some("mul"),
                _ => None,
            };

            if operation != None
            {
                i += 4;

                let mut op1 = String::new();
                let mut op2 = String::new();

                for x in i..input.len()
                {
                    match input[x] {
                        '0'..'9' => {
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
                            '0'..'9' => {
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

                        if operation.is_some_and(|x| x == "mul")
                        {
                            sum += one * two;
                        }
                    }
                }
            }
        }
    }

    println!("Result: {sum}");
}