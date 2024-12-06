fn lines_around1(x: usize, y: usize, xlen: usize, ylen: usize) -> Vec<[(usize, usize); 3]> {
    let mut ret = Vec::new();
    if x >= 3 {
        ret.push([(x - 1, y), (x - 2, y), (x - 3, y)]);
        if y >= 3 {
            ret.push([(x - 1, y - 1), (x - 2, y - 2), (x - 3, y - 3)]);
        }
        if y < ylen - 3 {
            ret.push([(x - 1, y + 1), (x - 2, y + 2), (x - 3, y + 3)]);
        }
    }
    if x < xlen - 3 {
        ret.push([(x + 1, y), (x + 2, y), (x + 3, y)]);
        if y >= 3 {
            ret.push([(x + 1, y - 1), (x + 2, y - 2), (x + 3, y - 3)]);
        }
        if y < ylen - 3 {
            ret.push([(x + 1, y + 1), (x + 2, y + 2), (x + 3, y + 3)]);
        }
    }
    if y >= 3 {
        ret.push([(x, y - 1), (x, y - 2), (x, y - 3)]);
    }
    if y < ylen - 3 {
        ret.push([(x, y + 1), (x, y + 2), (x, y + 3)]);
    }
    ret
}

#[aoc(day4, part1)]
pub fn part1(input: &str) -> u32 {
    let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let (xlen, ylen) = (lines[0].len(), lines.len());
    lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| line.iter().enumerate().map(move |(x, &c)| (x, y, c)))
        .filter(|&(_, _, c)| c == 'X')
        .flat_map(|(x, y, _)| lines_around1(x, y, xlen, ylen))
        .map(|[(x1, y1), (x2, y2), (x3, y3)]| {
            let is_xmas = lines[y1][x1] == 'M' && lines[y2][x2] == 'A' && lines[y3][x3] == 'S';
            is_xmas as u32
        })
        .sum()
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> u32 {
    let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let (xlen, ylen) = (lines[0].len(), lines.len());
    lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| line.iter().enumerate().map(move |(x, &c)| (x, y, c)))
        .filter(|&(x, y, c)| 0 < x && x < xlen - 1 && 0 < y && y < ylen - 1 && c == 'A')
        .map(|(x, y, _)| {
            [
                [(x - 1, y - 1), (x + 1, y + 1)],
                [(x - 1, y + 1), (x + 1, y - 1)],
            ]
        })
        .map(|[[(x1, y1), (x2, y2)], [(x3, y3), (x4, y4)]]| {
            let is_mas1 = lines[y1][x1] == 'M' && lines[y2][x2] == 'S';
            let is_sam1 = lines[y1][x1] == 'S' && lines[y2][x2] == 'M';
            let is_mas2 = lines[y3][x3] == 'M' && lines[y4][x4] == 'S';
            let is_sam2 = lines[y3][x3] == 'S' && lines[y4][x4] == 'M';
            ((is_mas1 || is_sam1) && (is_mas2 || is_sam2)) as u32
        })
        .sum()
}
