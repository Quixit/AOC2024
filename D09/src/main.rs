use std::fs;

fn get_input() -> Vec<usize> {

    let result: Vec<Vec<usize>> = fs::read_to_string("./D09/input.txt")
        .expect("File exists.").replace("\r\n", "\n")
        .split("\n")
        .map(|x| x.chars().map(|y| y as usize - 48).collect()) //48 = reliant on UTF8 and input being numbers
        .collect();

    result[0].clone()
}

struct File {
    pub is_file: bool,
    pub file_id: usize,
    pub size: usize
}

fn main() {
    let input = get_input();      
    let mut compacted: Vec<Option<usize>> = Vec::new();
    let mut decompressed: Vec<Option<usize>> = Vec::new();
    let mut is_file = true;
    let mut file_id:usize = 0;
    let mut files: Vec<File> = Vec::new();

    for item in input {
        files.push(File { file_id, is_file, size: item });

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

    let sum_one = get_hash(compacted);

    let len:isize = files.len().try_into().unwrap();
    let mut i: isize = len -1;

    while i >= 0 {
        let f:usize = i.try_into().unwrap();

        if files[f].is_file { //file.
            for s in 0..files.len() {
                if s >= f {
                    break;
                }

                if !files[s].is_file && files[s].size >= files[f].size { //free space.
                    if files[s].size == files[f].size {
                        files.swap(s, f); //same size. swap
                        break;
                    }
                    else { //bigger, move
                        files[s].size = files[s].size - files[f].size;
                        files[f].is_file =false;

                        files.insert(s, File { is_file: true, file_id: files[f].file_id.clone(), size: files[f].size.clone() });
                       
                        break;
                    }
                }
            }
        }

        i -= 1;
    }

    for item in files {
        if item.is_file {
            for _ in 0..item.size
            {
                decompressed.push(Option::from(item.file_id));
            }

            file_id += 1;
        }
        else {
            for _ in 0..item.size
            {
                decompressed.push(None);
            }
        }
    }

    let sum_two = get_hash2(decompressed);

    println!("Output 1 {sum_one}");
    println!("Output 2 {sum_two}");
}

fn get_hash (input: Vec<Option<usize>>) -> usize {
    let mut compacted = input.clone();
    let mut swap_index = compacted.len() -1;
    let mut sum: usize = 0;

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

    sum
}

fn get_hash2 (input: Vec<Option<usize>>) -> usize {
    let compacted = input.clone();
    let mut sum:usize = 0;

    for i in 0..compacted.len() {

        if compacted[i] != None {
            sum += i * compacted[i].unwrap();
        }
    }

    sum
}