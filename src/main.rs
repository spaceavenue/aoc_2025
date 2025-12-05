use std::env;

mod day01;
mod day02;
mod day03;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = &args[1].parse::<u64>().unwrap();
    match program {
        1 => {
            day01::calculate_password();
        }
        2 => {
            day02::add_invalid_p1();
            day02::add_invalid_p2();
        }
        3 => {
            day03::get_joltage();
        }
        _ => {
            todo!()
        }
    }
}