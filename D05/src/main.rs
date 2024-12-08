use std::{collections::HashMap, fs};

fn get_input() -> (HashMap<u64,Vec<u64>>, Vec<Vec<u64>>) {
    let contents = fs::read_to_string("./D05/input.txt")
        .expect("File exists.");

    let partitions: Vec<String> = contents.replace("\r\n", "\n").split("\n\n").map(|x| x.parse().unwrap() ).collect();

    let mut map: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut vec = Vec::new();

    let lines: Vec<String>  = partitions[0].split("\n").map(|x|String::from(x)).collect();

    for line in lines {
        let values: Vec<u64> = line.split("|").map(|x|x.trim().parse().unwrap()).collect();
        let result = map.get(&values[0]);
        let mut list: Vec<u64> = Vec::new();

        if result != None {
            list = result.unwrap().clone();
        }

        if !list.contains(&values[1]) {
            list.push(values[1]);
        }

        map.insert(values[0], list); //key = earlier value, value = list of values that must be after.
    }

    let lines: Vec<String>  = partitions[1].split("\n").map(|x| String::from(x)).collect();

    for line in lines {
        vec.push(line.split(",").map(|x|  x.parse().unwrap()).collect());
    }

    return  (map, vec);
}


fn main() {
    let (order_map, data) = get_input();
    let mut sum:u64 = 0;
    let mut invalid_sum: u64 = 0;

    for mut line in data {
        let mut set: HashMap<u64, usize> = HashMap::new(); 
        let mut valid = true;

        let mut i:usize = 0;

        while i < line.len() {
            

            let order = order_map.get(&line[i]);

            if order != None
            {
                let order = order.unwrap();
                for after in order
                {
                    if set.contains_key(&after) {
                        valid = false;

                        line.swap(i, *set.get(&after).unwrap());

                        i = 0;
                        set.clear();
                        continue;
                    }
                }
            }

            set.insert(line[i], i);
            i += 1;
        }

        if valid {
            sum += line[line.len()/2];
        } else {
            invalid_sum += line[line.len()/2];
        }
    }

    println!("Valid Sum: {sum}");
    println!("Invalid Sum: {invalid_sum}");
}