use std::{env, fs, vec};

pub fn get_ingredients_p1(ranges: &Vec<Vec<u64>>, ids: Vec<u64>) -> u64 {

    let mut fresh_ids = 0;

    for id in ids {
        for range in ranges {
            if (range[0]..=range[1]).contains(&id) {
                fresh_ids += 1;
                break
            }
        }
    }
    fresh_ids
}

fn get_ingredients_p2(ranges: &mut Vec<Vec<u64>>) -> u64 {
    let mut fresh_ids: u64 = 0;
    for idx in 0..ranges.len(){
        let start = ranges[idx][0];
        let end = ranges[idx][1];
        println!("i{idx}");
        for next_idx in (idx + 1)..ranges.len() - 1 {
            println!("{next_idx}");
            // if i can figure out how to make a vec of tuples this would be easier
            let next_start = ranges[next_idx][0];
            let next_end = ranges[next_idx][1];

            if start >= next_start && end <= next_end {
                ranges.remove(idx);
                continue;
            }
            if next_start >= start && next_end <= end {
                ranges.remove(next_idx);
                continue;
            }
            if next_start <= end && end <= next_end {
                let temp = vec![start.min(next_start), end.max(next_end)];
                ranges[idx] = temp;
                ranges.remove(next_idx);
                continue;
            }
        }
        fresh_ids += end - start + 1;
    }
    fresh_ids
}

pub fn get_ingredients() {
    let args: Vec<String> = env::args().collect();
    let file_name: &String = &args[2];

    let data = fs::read_to_string(file_name).expect("couldn't read file");
    let (ranges, ids) = data.split_once("\n\n").expect("something just happened.");

    let mut ranges = ranges.lines()
                                .map(|id_range| id_range.split('-')
                                    .map(|id| id.parse::<u64>().unwrap())
                                    .collect::<Vec<u64>>())
                                .collect::<Vec<Vec<u64>>>();

    let ids = ids.lines()
                            .map(|id| id.parse::<u64>().unwrap())
                            .collect::<Vec<u64>>();

    let fresh_ids_p1 = get_ingredients_p1(&ranges, ids);
    let fresh_ids_p2 = get_ingredients_p2(&mut ranges);

    println!("Fresh ids (part 1): {fresh_ids_p1}");
    println!("Fresh ids (part 2): {fresh_ids_p2}");
}