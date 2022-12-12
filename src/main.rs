#![feature(array_chunks)]
#![feature(iter_array_chunks)]
#![feature(portable_simd)]
#![feature(test)]

use std::fmt::Display;

mod day1;
mod day10;
mod day11;
mod day12;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

#[macro_export]
macro_rules! run {
    ($f:expr) => {{
        _run($f, stringify!($f));
    }};
}

fn _run<F, O>(f: F, name: &'static str)
where
    F: Fn() -> O,
    O: Display,
{
    let result = f();
    println!("{name} = {result}");
}

fn main() {
    run!(day1::part1);
    run!(day1::part2);
    run!(day2::part1);
    run!(day2::part2);
    run!(day3::part1);
    run!(day3::part2);
    run!(day3::part2_simd);
    run!(day4::part1);
    run!(day4::part2);
    run!(day5::part1);
    run!(day5::part2);
    run!(day6::part1);
    run!(day6::part2);
    run!(day7::part1);
    run!(day7::part2);
    run!(day8::part1);
    run!(day8::part2);
    run!(day9::part1);
    run!(day9::part2);
    run!(day10::part1);
    run!(day10::part2);
    run!(day11::part1);
    run!(day11::part2);
    run!(day12::part1);
    run!(day12::part2);
}
