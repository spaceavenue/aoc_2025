use std::{env, fs};

pub fn get_ingredients() {
    let args: Vec<String> = env::args().collect();
    let file_name: &String = &args[2];

    let data = fs::read_to_string(file_name).expect("couldn't read file");
    let (ranges, ids) = data.split_once("\n\n").expect("something just happened.");

    let ranges = ranges.lines()
                                .map(|id_range| id_range.split('-')
                                    .map(|id| id.parse::<u64>().unwrap())
                                    .collect::<Vec<u64>>())
                                .collect::<Vec<Vec<u64>>>();

    let ids = ids.lines()
                            .map(|id| id.parse::<u64>().unwrap())
                            .collect::<Vec<u64>>();
    
    println!("{:?}", ranges);
    println!("{:?}", ids);

    let mut fresh_ids = 0;

    for id in ids {
        for range in &ranges {
            if (range[0]..=range[1]).contains(&id) {
                // println!("found {id}");
                fresh_ids += 1;
                break
            }
        }
    }

    let mut fresh_ids_p2 = 0;

    for range in ranges {
        for _i in range[0]..=range[1] {
            fresh_ids_p2 += 1
        }
    }
    println!("{fresh_ids}");
    println!("{fresh_ids_p2}");
}