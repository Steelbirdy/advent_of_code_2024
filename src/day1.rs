#[aoc_generator(day1)]
pub fn generator(input: &str) -> Vec<(u32, u32)> {
    input
        .lines()
        .map(|line| (line[..5].parse().unwrap(), line[8..].parse().unwrap()))
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[(u32, u32)]) -> u32 {
    let (mut left, mut right): (Vec<_>, Vec<_>) = input.iter().copied().unzip();
    left.sort();
    right.sort();

    left.into_iter()
        .zip(right)
        .map(|(l, r)| u32::abs_diff(l, r))
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &[(u32, u32)]) -> u32 {
    let (mut left, mut right): (Vec<_>, Vec<_>) = input.iter().copied().unzip();
    left.sort();
    right.sort();

    let mut i = 0;
    let mut j = 0;
    let mut ret = 0;

    let mut last = 0;

    while i < left.len() {
        // Check if the number is the same as the last
        if i != 0 && left[i - 1] == left[i] {
            ret += last;
            i += 1;
            continue;
        }

        // Move past irrelevant numbers in `right`
        while right[j] < left[i] {
            j += 1;
        }

        // Count occurrences of `left[i]` in `right`
        let mut count = 0;
        while right[j] == left[i] {
            j += 1;
            count += 1;
        }

        // Store the value for possible use next iteration
        last = left[i] * count;
        ret += last;
        i += 1;
    }

    ret
}
