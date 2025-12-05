use std::{env, fs};


fn can_split(bank: &str, pending_bats: usize, idx: usize) -> bool {
    if pending_bats != 1 {
        if (bank.len() - idx) < pending_bats {
            return false;
        }
    }
    true
}

fn get_max(bank: &str, pending_bats: usize) -> (usize, usize) {
     //convert (usize, char) -> (usize, usize) idk if this is the best way to do this
    let mut batteries = bank.char_indices()
                                                                .map(|item| (item.0, (item.1 as u8 - 48) as usize));
    let mut max = 0;
    let mut max_idx = 0;

    while let Some(battery) = batteries.next() {
        if battery.1 > max && can_split(bank, pending_bats, battery.0) {
            max = battery.1;
            max_idx = battery.0;
        }
    }
    (max_idx, max)
}

fn find_highest(bank: &str, pending_bats: usize) -> usize {
    if pending_bats == 0 { return 0; }

    let (max_idx, mut max) = get_max(bank, pending_bats);
    max = max * 10_usize.pow((pending_bats - 1) as u32);
    let new = bank.split_at(max_idx + 1).1;
    
    return max + find_highest(new, pending_bats - 1);
}

pub fn get_joltage() {
    let args: Vec<String> = env::args().collect();
    let file_name: &String = &args[2];
    
    let data = fs::read_to_string(file_name).expect("Unable to read file.");
    let banks: Vec<&str> = data.lines().collect();

    let total_joltage_p1 = banks.iter().map(|bank| find_highest(bank, 2))
                                    .fold(0_usize, |total_joltage, current| total_joltage + current);
    println!("Total joltage (part 1): {total_joltage_p1}");

    let total_joltage_p2 = banks.iter().map(|bank| find_highest(bank, 12))
                                    .fold(0_usize, |total_joltage, current| total_joltage + current);
    println!("Total joltage (part 2): {total_joltage_p2}");
}