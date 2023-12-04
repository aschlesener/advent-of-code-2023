use std::env;
use std::process;

use advent_of_code_2023::AdventDay;
use advent_of_code_2023::day1;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day = AdventDay::build(&args).unwrap_or_else(| err  | {
        eprintln!("Problem parsing args: {err}");
        process::exit(1);
    });

    match day.number {
        1 => day1(),
        _ => println!("Day not implemented"),
    }

}
