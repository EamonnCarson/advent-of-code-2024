mod day;
mod display;

use clap::Parser;
use colored::Colorize;
use std::{env, fs, time::Instant};

use crate::{
    day::Day,
    display::{
        logger::{log, YuleLogger},
        panic_handler::naughty_list_panic_handler,
    },
};

mod global_imports {
    pub(crate) use crate::{
        day::Day,
        display::logger::{debug, log},
    };
    pub use lazy_static::lazy_static;
    pub use std::{convert::Infallible, str::FromStr};
}

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    stress_test: bool,
    #[arg(short = 'l', long, default_value_t = 1000)]
    test_len: u32,
    day_num: usize,
}

lazy_static::lazy_static! {
    static ref ARGS: Args = Args::parse();
}

fn main() {
    YuleLogger::init();
    std::panic::set_hook(Box::new(naughty_list_panic_handler));
    let day_num = ARGS.day_num;

    let input = get_input(day_num);

    let day = get_day(day_num);

    println!("\n{}\n", display::santa_hat());

    let to_run = match ARGS.stress_test {
        false => run_star,
        true => {
            log!("Stress Testing over {} runs", ARGS.test_len);
            stress_test_star
        }
    };

    to_run(day, input.clone(), false);
    to_run(day, input, true)
}

fn get_input(day_num: usize) -> String {
    let cwd = env::current_dir().unwrap();
    let filename = cwd.join("inputs").join(format!("day{:02}.txt", day_num));
    log!("Reading {}\n", filename.display());
    fs::read_to_string(&filename)
        .unwrap_or_else(|e| panic!("Error While reading input for day {day_num}. Expected input file at {}: {e}", filename.display()))
}

fn run_star(day: &dyn Day, input: String, is_second_star: bool) {
    let day_num = is_second_star as usize + 1;

    println!(
        "{} {} {}",
        "Running".green(),
        "*".bright_yellow(),
        day_num.to_string().green()
    );
    println!("{}", display::banner());
    let start = Instant::now();
    let solution = match is_second_star {
        false => day.star1(input),
        true => day.star2(input),
    };
    let dur = start.elapsed();
    let test_solution = match is_second_star {
        false => day.test_star1(),
        true => day.test_star2(),
    };
    println!("{} {test_solution}", "Test:".green());
    println!();
    println!("{} {solution}", "Solution:".green());
    println!("{}: {dur:?}", "Took".green());
    println!();
}

fn stress_test_star(day: &dyn Day, input: String, is_second_star: bool) {
    let day_num = is_second_star as usize + 1;
    let mut total = std::time::Duration::ZERO;
    for _ in 0..ARGS.test_len {
        total += time_star(day, input.clone(), is_second_star);
    }
    log!("Ran star {} in {:?}", day_num, total / ARGS.test_len);
}

fn time_star(day: &dyn Day, input: String, is_second_star: bool) -> std::time::Duration {
    let start = std::time::Instant::now();
    match is_second_star {
        false => day.star1(input),
        true => day.star2(input),
    };
    start.elapsed()
}

macro_rules! build_get_day {
    ($($days:tt)*) => {
        fn get_day(day_num: usize) -> &'static dyn Day {
            match day_num {
                $(
                    #[allow(clippy::zero_prefixed_literal)]
                    $days => &paste::paste!(day::[<day $days>]::[<Day $days>]) as &dyn Day,
                )*
                _ => panic!("Day {day_num} not in the christmas spirit")
            }
        }
    };
}

build_get_day! { 01 02 03 04 05 06 07 08 09 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 }
