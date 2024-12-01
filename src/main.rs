use std::{env, path::Path, process};

mod days;

use days::*;

fn main() {
    let day = env::args()
        .skip(1)
        .next()
        .unwrap_or_else(|| {
            println!("Provide a day to execute T_T");
            process::exit(1)
        })
        .trim()
        .parse::<u32>()
        .unwrap_or_else(|_| {
            println!("Provide the day as a number O.O");
            process::exit(1)
        });

    if !Path::new("inputs").exists() {
        println!("Create an inputs/ directory and fill it with your puzzle inputs ^-^");
        process::exit(1)
    }

    match day {
        0 => println!("Not a day -w-"),
        1 => day_1::run(),
        2 => day_2::run(),
        3 => day_3::run(),
        4 => day_4::run(),
        5 => day_5::run(),
        6 => day_6::run(),
        7 => day_7::run(),
        8 => day_8::run(),
        9 => day_9::run(),
        10 => day_10::run(),
        11 => day_11::run(),
        12 => day_12::run(),
        13 => day_13::run(),
        14 => day_14::run(),
        15 => day_15::run(),
        16 => day_16::run(),
        17 => day_17::run(),
        18 => day_18::run(),
        19 => day_19::run(),
        20 => day_20::run(),
        21 => day_21::run(),
        22 => day_22::run(),
        23 => day_23::run(),
        24 => day_24::run(),
        25 => day_25::run(),
        _ => println!("Day too big @w@"),
    }
}
