use std::env;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = &args[1].parse::<u64>().unwrap();
    match program {
        1 => {
            day01::calculate_password();
        }
        2 => {
            day02::add_invalid();
        }
        3 => {
            day03::get_joltage();
        }
        4 => {
            day04::get_toilet_paper();
        }
        5 => {
            day05::get_ingredients();
        }
        _ => {
            todo!()
        }
    }
}