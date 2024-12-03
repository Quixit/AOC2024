use std::fs;
// use std::collections::HashMap;

fn get_input() -> Vec<Vec<i64>> {
    let contents = fs::read_to_string("./D02/input.txt")
        .expect("File exists.");

    return contents.replace("\r\n", "\n")
        .split("\n")
        .map(|i|String::from(i).split(" ").map(|x: &str| x.parse().expect("Not number.")).collect())
        .collect();
}

fn main() {
    let input = get_input();

    let mut sum: u64 = 0;
    let mut buffer_sum: u64 = 0;

    for item in input.clone().into_iter()
    {
        let result = evaluate_line(&item);

        if result {
            sum += 1;
        }
        else {
            for i in 0..item.len()
            {
                let mut item2 = item.clone();
                item2.remove(i);

                if evaluate_line(&item2) {
                    buffer_sum += 1;

                    break;
                }
            }
        }
    }

    let total = sum + buffer_sum;

    println!("Safe: {sum}");
    println!("Buffered Safe: {total}");
}

fn evaluate_line(item: &Vec<i64>) -> bool {
    let increasing = item[0] < item[1];

    for i in 1..item.len()
    {
        let diff: i64;

        if increasing {
            diff = item[i] - item[i-1];
        }
        else {
            diff = item[i-1] - item[i];
        }

        if diff < 1 || diff > 3 {
            return false;
        }
    }

    return true;
}