mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

#[allow(dead_code)]
mod template;

use std::{env, process};

const DAY_FUNCS: &[fn()] = &[day1::run, day2::run, day3::run, day4::run, day5::run];

fn main() {
    let mut args = env::args();
    let program_name = args.next().expect("program name");

    // get the day index
    let index = match args.next() {
        Some(index) => match index.parse::<usize>() {
            Ok(index) => index,
            Err(_) => {
                eprintln!("Error: day must be a number");
                process::exit(1);
            }
        },
        None => {
            eprintln!("Usage: {program_name} <day> <input_files>");
            process::exit(1);
        }
    };

    if let Some(day_func) = DAY_FUNCS.get(index.wrapping_sub(1)) {
        day_func();
    } else {
        eprintln!("Error: day {index} is not implemented");
        process::exit(1);
    }
}
