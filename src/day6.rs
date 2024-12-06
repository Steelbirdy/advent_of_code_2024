type Map = Vec<Vec<u8>>;

#[derive(Copy, Clone)]
struct Guard {
    pos: (usize, usize),
    dir: (isize, isize),
}

impl Guard {
    fn new(map: &Map) -> Option<Self> {
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                let dir = match map[y][x] {
                    b'^' => (0, -1),
                    b'>' => (1, 0),
                    b'v' => (0, 1),
                    b'<' => (-1, 0),
                    _ => continue,
                };
                return Some(Self { pos: (x, y), dir });
            }
        }
        None
    }

    fn rotate_right(&mut self) {
        self.dir = (-self.dir.1, self.dir.0);
    }

    fn do_move(&mut self, map: &Map) {
        let (mut nx, mut ny) = self.next_position();
        while is_in_bounds(nx, ny, map) && map[ny][nx] == b'#' {
            self.rotate_right();
            (nx, ny) = self.next_position();
        }
        self.pos = self.next_position();
    }

    fn next_position(&self) -> (usize, usize) {
        (
            self.pos.0.wrapping_add_signed(self.dir.0),
            self.pos.1.wrapping_add_signed(self.dir.1),
        )
    }

    fn is_in_bounds(&self, map: &Map) -> bool {
        is_in_bounds(self.pos.0, self.pos.1, map)
    }
}

fn is_in_bounds(x: usize, y: usize, map: &Map) -> bool {
    x < map[0].len() && y < map.len()
}

#[derive(Default, Copy, Clone)]
#[repr(transparent)]
struct History(u8);

impl History {
    fn contains(&self, dx: isize, dy: isize) -> bool {
        self.0 & Self::byte(dx, dy) != 0
    }

    fn insert(&mut self, dx: isize, dy: isize) {
        self.0 |= Self::byte(dx, dy);
    }

    fn byte(dx: isize, dy: isize) -> u8 {
        match (dx, dy) {
            (0, -1) => 0b0001,
            (0, 1) => 0b0010,
            (-1, 0) => 0b0100,
            (1, 0) => 0b1000,
            _ => unreachable!(),
        }
    }
}

#[aoc_generator(day6)]
pub fn generator(input: &str) -> Map {
    input.lines().map(|l| l.bytes().collect()).collect()
}

#[aoc(day6, part1)]
pub fn part1(map: &Map) -> u32 {
    let mut visited = crate::HashSet::default();
    let mut guard = Guard::new(map).unwrap();
    while guard.is_in_bounds(map) {
        visited.insert(guard.pos);
        guard.do_move(map);
    }
    visited.len() as _
}

#[aoc(day6, part2)]
pub fn part2(map: &Map) -> u32 {
    let mut map = map.clone();

    let mut visited = crate::HashSet::default();
    let mut guard = Guard::new(&map).unwrap();
    let starting_guard = guard;
    while guard.is_in_bounds(&map) {
        visited.insert(guard.pos);
        guard.do_move(&map);
    }

    let mut result = 0;
    let mut history = vec![vec![History::default(); map[0].len()]; map.len()];
    for (vx, vy) in visited {
        if vx == starting_guard.pos.0 && vy == starting_guard.pos.1 {
            continue;
        }
        map[vy][vx] = b'#';
        let mut guard = starting_guard;
        while guard.is_in_bounds(&map) {
            let (x, y) = guard.pos;
            let (dx, dy) = guard.dir;
            if history[y][x].contains(dx, dy) {
                result += 1;
                break;
            }
            history[y][x].insert(dx, dy);
            guard.do_move(&map);
        }
        map[vy][vx] = b'.';
        history.iter_mut().flatten().for_each(|h| {
            *h = History::default();
        });
    }

    result
}
