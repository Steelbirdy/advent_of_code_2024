use crate::{HashMap, HashSet};
use itertools::Itertools;

type Input = Vec<Vec<u8>>;

#[aoc_generator(day8)]
pub fn generator(input: &str) -> Input {
    input.lines().map(|l| l.bytes().collect()).collect()
}

#[aoc(day8, part1)]
pub fn part1(input: &Input) -> u64 {
    let x_len = input[0].len();
    let y_len = input.len();

    let mut antennas: HashMap<u8, Vec<(usize, usize)>> = HashMap::default();
    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == b'.' {
                continue;
            }
            antennas.entry(*c).or_default().push((x, y));
        }
    }

    let mut antinodes = HashSet::default();
    for locations in antennas.into_values() {
        for pair in locations.into_iter().combinations(2) {
            let (x1, y1) = pair[0];
            let (x2, y2) = pair[1];
            'ba: {
                let Some(xa) = (2 * x2).checked_sub(x1) else {
                    break 'ba;
                };
                let Some(ya) = (2 * y2).checked_sub(y1) else {
                    break 'ba;
                };
                if xa < x_len && ya < y_len {
                    antinodes.insert((xa, ya));
                }
            }
            'bb: {
                let Some(xb) = (2 * x1).checked_sub(x2) else {
                    break 'bb;
                };
                let Some(yb) = (2 * y1).checked_sub(y2) else {
                    break 'bb;
                };
                if xb < x_len && yb < y_len {
                    antinodes.insert((xb, yb));
                }
            }
        }
    }

    antinodes.len() as _
}

#[aoc(day8, part2)]
pub fn part2(input: &Input) -> u64 {
    let x_len = input[0].len();
    let y_len = input.len();

    let add_inbounds_x = |a: usize, b: isize| a.checked_add_signed(b).filter(|x| *x < x_len);
    let add_inbounds_y = |a: usize, b: isize| a.checked_add_signed(b).filter(|y| *y < y_len);

    let mut antennas: HashMap<u8, Vec<(usize, usize)>> = HashMap::default();
    let mut antinodes = HashSet::default();
    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == b'.' {
                continue;
            }
            antennas.entry(*c).or_default().push((x, y));
            antinodes.insert((x, y));
        }
    }

    for locations in antennas.into_values() {
        for pair in locations.into_iter().combinations(2) {
            let (x1, y1) = pair[0];
            let (x2, y2) = pair[1];

            let dx = x2 as isize - x1 as isize;
            let dy = y2 as isize - y1 as isize;

            let (mut xa, mut ya) = (x1, y1);
            let (mut xb, mut yb) = (x2, y2);

            loop {
                let Some(xa_) = add_inbounds_x(xa, -dx) else {
                    break;
                };
                let Some(ya_) = add_inbounds_y(ya, -dy) else {
                    break;
                };
                xa = xa_;
                ya = ya_;
                antinodes.insert((xa, ya));
            }
            loop {
                let Some(xb_) = add_inbounds_x(xb, dx) else {
                    break;
                };
                let Some(yb_) = add_inbounds_y(yb, dy) else {
                    break;
                };
                xb = xb_;
                yb = yb_;
                antinodes.insert((xb, yb));
            }
        }
    }

    antinodes.len() as _
}
