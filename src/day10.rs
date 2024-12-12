use rayon::prelude::*;

#[aoc_generator(day10)]
pub fn generator(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.bytes().collect()).collect()
}

#[aoc(day10, part1)]
pub fn part1(input: &[Vec<u8>]) -> u64 {
    let init = || (crate::HashSet::default(), DfsRunner::new(input));
    trailheads(input)
        .map_init(init, |(seen, runner), (x, y)| {
            runner.dfs(x, y, |x, y| {
                seen.insert((x, y));
            });
            let res = seen.len() as u64;
            seen.clear();
            res
        })
        .sum()
}

#[aoc(day10, part2)]
pub fn part2(input: &[Vec<u8>]) -> u64 {
    let init = || DfsRunner::new(input);
    trailheads(input)
        .map_init(init, |runner, (x, y)| {
            let mut res = 0;
            runner.dfs(x, y, |_, _| {
                res += 1;
            });
            res
        })
        .sum()
}

fn trailheads(input: &[Vec<u8>]) -> impl ParallelIterator<Item = (usize, usize)> + '_ {
    (0..input[0].len())
        .into_par_iter()
        .flat_map(|x| (0..input.len()).into_par_iter().map(move |y| (x, y)))
        .filter(|&(x, y)| input[y][x] == b'0')
}

struct DfsRunner<'a> {
    input: &'a [Vec<u8>],
    queue: std::collections::VecDeque<(usize, usize)>,
}

impl<'a> DfsRunner<'a> {
    fn new(input: &'a [Vec<u8>]) -> Self {
        Self {
            input,
            queue: std::collections::VecDeque::new(),
        }
    }

    fn dfs<F>(&mut self, x: usize, y: usize, mut callback: F)
    where
        F: FnMut(usize, usize),
    {
        const DELTA: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        self.queue.push_back((x, y));
        while let Some((x, y)) = self.queue.pop_front() {
            let height = self.input[y][x];
            if height == b'9' {
                callback(x, y);
                continue;
            }
            for (dx, dy) in DELTA {
                let (x2, y2) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
                let height2 = self.input.get(y2).and_then(|line| line.get(x2)).copied();
                if height2 == Some(height + 1) {
                    self.queue.push_back((x2, y2));
                }
            }
        }
    }
}
