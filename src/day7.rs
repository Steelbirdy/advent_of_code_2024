use rayon::prelude::*;

#[aoc_generator(day7)]
pub fn generator(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .map(|l| {
            let (test, values) = l.split_once(':').unwrap();
            let values = values
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<_>>();
            (test.parse().unwrap(), values)
        })
        .collect()
}

fn check1(test: u64, values: &[u64]) -> bool {
    let mut results = vec![test];
    let mut new_results = Vec::new();
    for &value in values.iter().rev() {
        for res in results.drain(..) {
            if res % value == 0 {
                new_results.push(res / value);
            }
            if let Some(res) = res.checked_sub(value) {
                new_results.push(res);
            }
        }
        std::mem::swap(&mut results, &mut new_results);
    }
    results.iter().any(|&x| x == 0)
}

fn check2(test: u64, values: &[u64]) -> bool {
    let mut results = vec![test];
    let mut new_results = Vec::new();
    for &value in values.iter().rev() {
        for res in results.drain(..) {
            if res % value == 0 {
                new_results.push(res / value);
            }
            let vlog = 10_u64.pow(value.ilog10() + 1);
            if res % vlog == value {
                new_results.push(res / vlog);
            }
            if let Some(res) = res.checked_sub(value) {
                new_results.push(res);
            }
        }
        std::mem::swap(&mut results, &mut new_results);
    }
    results.iter().any(|&x| x == 0)
}

#[aoc(day7, part1)]
pub fn part1(input: &[(u64, Vec<u64>)]) -> u64 {
    input
        .par_iter()
        .filter_map(|(test, values)| check1(*test, values).then_some(*test))
        .sum()
}

#[aoc(day7, part2)]
pub fn part2(input: &[(u64, Vec<u64>)]) -> u64 {
    input
        .par_iter()
        .filter_map(|(test, values)| check2(*test, values).then_some(*test))
        .sum()
}
