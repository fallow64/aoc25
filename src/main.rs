mod day1;
mod day2;
mod day3;

use std::{env, fs, process};

const DAY_FUNCS: &[fn(Vec<String>)] = &[day1::run, day2::run, day3::run];

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

    let file_names = args.collect::<Vec<String>>();

    let files = file_names
        .iter()
        .map(|file_name| {
            fs::read_to_string(file_name).unwrap_or_else(|_| {
                eprintln!("Error: could not read file '{file_name}'");
                process::exit(1);
            })
        })
        .collect::<Vec<String>>();

    if let Some(day_func) = DAY_FUNCS.get(index.wrapping_sub(1)) {
        day_func(files);
    } else {
        eprintln!("Error: day {index} is not implemented");
        process::exit(1);
    }
}
