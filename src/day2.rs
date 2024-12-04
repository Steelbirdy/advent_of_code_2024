#[aoc_generator(day2)]
pub fn generator(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.split(' ').map(|x| x.parse().unwrap()).collect())
        .collect()
}

fn check(line: &[u32], ignore: Option<usize>) -> bool {
    let mut x = line[0];
    let mut y = line[1];
    if ignore == Some(0) {
        x = y;
        y = line[2];
    } else if ignore == Some(1) {
        y = line[2];
    }

    let incr = if x == y {
        return false;
    } else {
        x < y
    };

    line.windows(2)
        .enumerate()
        .map(|(i, w)| {
            let &[x, mut y] = w else { unreachable!() };
            if Some(i) == ignore {
                return true;
            } else if Some(i + 1) == ignore {
                if i + 1 == line.len() - 1 {
                    return true;
                }
                y = line[i + 2];
            }

            if (incr && x > y) || (!incr && x < y) {
                return false;
            }

            let diff = x.abs_diff(y);
            1 <= diff && diff <= 3
        })
        .all(|x| x)
}

#[aoc(day2, part1)]
pub fn part1(input: &[Vec<u32>]) -> u32 {
    input
        .iter()
        .map(|line| check(line, None))
        .filter(|&x| x)
        .count() as _
}

#[aoc(day2, part2)]
pub fn part2(input: &[Vec<u32>]) -> u32 {
    input
        .iter()
        .map(|line| {
            (0..line.len())
                .map(|i| check(line, Some(i)))
                .chain(std::iter::once(check(line, None)))
                .any(|x| x)
        })
        .filter(|x| *x)
        .count() as _
}
