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

// criterion_group!(benches, day5_bench, day6_bench, day7_bench, day8_bench);
criterion_group!(benches, day8_bench);
criterion_main!(benches);