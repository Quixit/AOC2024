use std::{collections::HashMap, fs};
// use std::collections::HashMap;

fn get_input() -> (HashMap<u64,Vec<u64>>, Vec<Vec<u64>>) {
    let contents = fs::read_to_string("./D05/input.txt")
        .expect("File exists.");

    let partitions: Vec<String> = contents.replace("\r\n", "\n").split("\n\n").map(|x| x.parse().unwrap() ).collect();

    let mut map: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut vec = Vec::new();

    let lines: Vec<String>  = partitions[0].split("\n\n").map(|x|String::from(x)).collect();

    for line in lines {
        let values: Vec<u64> = line.split("|").map(|x|x.parse().unwrap()).collect();
        let result = map.get(&values[1]);
        let mut list: Vec<u64> = Vec::new();

        if result != None {
            list = result.unwrap().clone();
        }

        if !list.contains(&values[0]) {
            list.push(values[0]);
        }

        map.insert(values[1], list); //key = later value, value = list of values that must be before.
    }

    let lines: Vec<String>  = partitions[1].split("\n\n").map(|x| String::from(x)).collect();

    for line in lines {
        vec.push(line.split(",").map(|x|  x.parse().unwrap()).collect());
    }

    return  (map, vec);
}


fn main() {
    let (order_map, data) = get_input();
    let mut sum:u64 = 0;
    let mut mas_count:u64 = 0;

    println!("Middle Sum: {sum}");
}