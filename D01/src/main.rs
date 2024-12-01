use std::fs;
use std::collections::HashMap;

fn get_input() -> Vec<Vec<u64>> {
    let contents = fs::read_to_string("./D01/input.txt")
        .expect("File exists.");

    return contents.replace("\r\n", "")
        .split("\n")
        .map(|i|String::from(i).split("   ").map(|x: &str| x.parse().expect("Not number.")).collect())
        .collect();
}

fn main() {
    let input = get_input();

    let mut left: Vec<u64> = input.clone().into_iter().map(|i| i[0]).collect();
    let mut right: Vec<u64> = input.into_iter().map(|i| i[1]).collect();

    left.sort();
    right.sort();

    let mut sum: u64 = 0;
    let mut similarity: u64 = 0;
    let mut grouped_right: HashMap<u64, u64> = HashMap::new(); 

    for i in 0..left.len()
    {
        sum += left[i].abs_diff(right[i]);

        *grouped_right.entry(right[i]).or_insert(0) += 1;
    }

    for i in 0..left.len()
    {
        let default:u64 = 0;
        similarity += left[i] * grouped_right.get(&left[i]).or(Option::from(&default)).expect("Key not missing.");
    }

   println!("Sum of differences: {sum}");
   println!("Similarity: {similarity}");
}
