use color_eyre::eyre::Result;

use aoc2022::Aoc;

// use clap::{Parser, ValueEnum};

macro_rules! run_day {
    ($day: ident) => {
        let s = stringify!($day).to_ascii_lowercase();

        println!("=== {s} ===");
        println!("dummy input:");
        aoc2022::$day.run(&format!("input/{s}.dummy.txt"))?;
        println!("personal input:");
        aoc2022::$day.run(&format!("input/{s}.txt"))?;
    };
}

fn main() -> Result<()> {
    // run_day!(Day01);
    // run_day!(Day02);
    // run_day!(Day03);
    // run_day!(Day04);
    // run_day!(Day05);
    // run_day!(Day06);
    // run_day!(Day07);
    // run_day!(Day08);
    // run_day!(Day09);
    // run_day!(Day10);
    // run_day!(Day11);
    // run_day!(Day12);
    // run_day!(Day13);
    // run_day!(Day14);
    run_day!(Day15);

    Ok(())
}
