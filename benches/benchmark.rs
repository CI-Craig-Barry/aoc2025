extern crate core;

// use std::fmt::Display;
use criterion::{Criterion, criterion_group, criterion_main};
use aoc2025::utils;
#[path = "../src/bin/day4/main.rs"]
mod day4;
#[path = "../src/bin/day5/main.rs"]
mod day5;
#[path = "../src/bin/day6/main.rs"]
mod day6;
#[path = "../src/bin/day7/main.rs"]
mod day7;
#[path = "../src/bin/day8/main.rs"]
mod day8;
#[path = "../src/bin/day9/main.rs"]
mod day9;
#[path = "../src/bin/day10/main.rs"]
mod day10;
#[path = "../src/bin/day11/main.rs"]
mod day11;
#[path = "../src/bin/day12/main.rs"]
mod day12;

fn benchmark(
  crit: &mut Criterion,
  details: &utils::ExecDetails
) {
  let mut group = crit.benchmark_group(format!("day{}", details.day));
  group.bench_function("task1", |benchmark| {
    benchmark.iter(|| (details.task1_function)(&details.input))
  });
  group.bench_function("task2", |benchmark| {
    benchmark.iter(|| (details.task2_function)(&details.input))
  });
}

fn day4_bench(crit: &mut Criterion)
{
  benchmark(crit, &day4::get_details());
}

fn day5_bench(crit: &mut Criterion)
{
  benchmark(crit, &day5::get_details());
}

fn day6_bench(crit: &mut Criterion)
{
  benchmark(crit, &day6::get_details());
}

fn day7_bench(crit: &mut Criterion)
{
  benchmark(crit, &day7::get_details());
}

fn day8_bench(crit: &mut Criterion)
{
  benchmark(crit, &day8::get_details());
}

fn day9_bench(crit: &mut Criterion)
{
  benchmark(crit, &day9::get_details());
}

fn day10_bench(crit: &mut Criterion)
{
  benchmark(crit, &day10::get_details());
}

fn day11_bench(crit: &mut Criterion)
{
  benchmark(crit, &day11::get_details());
}

fn day12_bench(crit: &mut Criterion)
{
  benchmark(crit, &day12::get_details());
}

// criterion_group!(benches, day4_bench, day5_bench, day6_bench, day7_bench, day8_bench, day9_bench, day10_bench, day11_bench, day12_bench);
criterion_group!(benches, day12_bench);
criterion_main!(benches);