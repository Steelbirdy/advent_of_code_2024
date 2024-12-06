regex!(MUL_INSTR = r"mul\((\d+),(\d+)\)");
regex!(DO_INSTR = r"do(?:n't)?\(\)");

fn evaluate(s: &str) -> u32 {
    MUL_INSTR
        .captures_iter(s)
        .map(|cap| cap[1].parse::<u32>().unwrap() * cap[2].parse::<u32>().unwrap())
        .sum()
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u32 {
    evaluate(input)
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u32 {
    let mut last_start = Some(0_usize);

    let enabled_ranges = DO_INSTR.find_iter(input).filter_map(|m| {
        if m.len() == 7 {
            if let Some(start) = last_start.take() {
                return Some(start..m.start());
            }
        } else if last_start.is_none() {
            last_start = Some(m.end());
        }
        None
    });

    let mut result = 0_u32;
    for range in enabled_ranges {
        result += evaluate(&input[range]);
    }
    if let Some(start) = last_start {
        result += evaluate(&input[start..]);
    }
    result
}
