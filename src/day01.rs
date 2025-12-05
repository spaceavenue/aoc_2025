use std::{env, fs};

const MAX_POS: isize = 100;

fn get_rotations() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    let file_name: &String = &args[2];
    
    let rotations = fs::read_to_string(file_name).expect("Unable to read file.");
    let vec_rotations: Vec<String> = rotations.lines().map(String::from).collect();

    return vec_rotations
}

fn parse_rotations(rotation: String) -> (char, isize) {
    let mut rotation_chars = rotation.chars();
    let dir = rotation_chars.next().unwrap();
    let amt = rotation_chars.collect::<String>().parse::<isize>().unwrap();
    
    return (dir, amt);
}

pub fn calculate_password() {
    let mut zero_count: usize = 0;
    let mut pass_count: usize = 0;
    let mut dial_pos: isize = 50;
    
    let rotations = get_rotations();
    
    for rotation in rotations {
        let (dir, mut amt) = parse_rotations(rotation);

        let mut complete_rotation = (amt / MAX_POS) as usize;
        amt = amt % MAX_POS;

        match dir {
            'R' => {
                if (dial_pos + amt) > MAX_POS {
                    if dial_pos != 100 {
                        complete_rotation += 1;
                    }
                    dial_pos = (dial_pos + amt) - MAX_POS;
                    println!("R {dial_pos}");
                }
                else {
                    dial_pos = dial_pos + amt;
                    println!("R {dial_pos}")

                }
            }
            'L' => {
                if (dial_pos - amt) < 0 {
                    if dial_pos != 0 {
                        complete_rotation += 1;
                    }
                    dial_pos = MAX_POS - (amt - dial_pos);
                    println!("L {dial_pos}");
                }
                else {
                    dial_pos = dial_pos - amt;
                    println!("L {dial_pos}");
                }
            }
            _ => panic!()
        }
        pass_count += complete_rotation;
        if dial_pos == 0 || dial_pos == 100 {
            zero_count += 1;
        }
    }
    println!("Password (part 1): {zero_count}");
    zero_count += pass_count;
    println!("Password (part 2): {zero_count}");
}