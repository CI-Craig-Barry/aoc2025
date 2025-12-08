// use std::fmt::Display;
use criterion::{Criterion, criterion_group, criterion_main};
use aoc2025::utils;

#[path = "../src/bin/day5/main.rs"]
mod day5;
#[path = "../src/bin/day6/main.rs"]
mod day6;

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

fn day5_bench(crit: &mut Criterion)
{
  benchmark(crit, &day5::get_details());
}

fn day6_bench(crit: &mut Criterion)
{
  benchmark(crit, &day6::get_details());
}

criterion_group!(benches, day5_bench, day6_bench);
criterion_main!(benches);