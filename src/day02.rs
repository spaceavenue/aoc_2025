use std::{env, fs};

pub fn read_id() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    let file_name: &String = &args[1];

    let ranges: Vec<String> = fs::read_to_string(file_name)
                                .expect("Unable to read file.")
                                .split(',')
                                .map(|range| range.trim_end_matches('\n'))
                                .map(String::from)
                                .collect();
    ranges
}

fn separate_id(ids: String) -> (usize, usize) {
    let mut ids = ids.split('-').map(|id| id.parse::<usize>().unwrap());
    let (first_id, last_id) = (ids.next().unwrap(), ids.next().unwrap());
    (first_id, last_id)
}

fn check_substrings(id_str: &str, num: usize) -> bool {
    if id_str.len() % num != 0 { return false }

    let substring_len = id_str.len() / num;
    let substring = &id_str[0..substring_len];

    for i in 0..num {
        if &id_str[(i * substring_len)..((i + 1) * substring_len)] != substring {
            return false
        }
    }
    true
}

pub fn add_invalid_p1() {
    let mut sum_invalid: usize = 0;

    let vec_ranges = read_id();
    
    for range in vec_ranges {
        let (first_id, last_id) = separate_id(range);
        for id in first_id..=last_id {
            let id_str = id.to_string();
            if check_substrings(&id_str, 2) {
                sum_invalid += id;
            }
        }
    }
    println!("Sum of invalid IDs (part 1): {sum_invalid}");
}

pub fn add_invalid_p2() {
    let mut sum_invalid: usize = 0;

    let vec_ranges = read_id();
    
    for range in vec_ranges {
        let (first_id, last_id) = separate_id(range);
        
        for id in first_id..=last_id {
            let id_str = id.to_string();
            let num_substring = 2..=id_str.len();

            for num in num_substring {
                if check_substrings(&id_str, num) {
                    sum_invalid += id;
                    break
                }
            }
        }
    }
    println!("Sum of invalid IDs (part 2): {sum_invalid}");
}