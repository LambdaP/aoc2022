use color_eyre::eyre::Result;

use aoc2022::Aoc;

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
    run_day!(Day1);
    run_day!(Day2);
    run_day!(Day3);
    run_day!(Day4);
    run_day!(Day5);
    run_day!(Day6);
    run_day!(Day7);
    run_day!(Day8);
    run_day!(Day9);
    run_day!(Day10);

    Ok(())
}
