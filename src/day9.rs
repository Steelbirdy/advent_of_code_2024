type Input = Vec<u8>;

#[aoc_generator(day9)]
pub fn generator(input: &str) -> Input {
    input.bytes().map(|b| b - b'0').collect()
}

fn triangle_sum(m: u64, n: u64) -> u64 {
    let m_minus_1 = m.saturating_sub(1);
    (n * (n - 1) - m * m_minus_1) / 2
}

fn compute(ptr: usize, index: u64, count: u64) -> u64 {
    (ptr as u64 / 2) * triangle_sum(index, index + count)
}

#[aoc(day9, part1)]
pub fn part1(input: &Input) -> u64 {
    let mut res = 0;
    let mut index = 0;
    let (mut front_ptr, mut back_ptr) = (0, input.len() + 1);
    let mut back_count = 0;

    while front_ptr < back_ptr {
        let count = input[front_ptr] as u64;
        res += compute(front_ptr, index, count);
        index += count;
        front_ptr += 1;
        let mut empty_space = input[front_ptr] as u64;
        while empty_space != 0 {
            if back_count == 0 {
                back_ptr -= 2;
                back_count = input[back_ptr] as u64;
            }
            if empty_space >= back_count {
                res += compute(back_ptr, index, back_count);
                index += back_count;
                empty_space -= back_count;
                back_count = 0;
            } else {
                res += compute(back_ptr, index, empty_space);
                index += empty_space;
                back_count -= empty_space;
                empty_space = 0;
            }
        }
        front_ptr += 1;
    }
    if front_ptr == back_ptr && back_count != 0 {
        res += compute(back_ptr, index, back_count);
    }

    res
}

#[derive(Copy, Clone)]
struct Block {
    size: u64,
    id: Option<u64>,
}

// TODO: Find an analogue to part1's efficient algorithm for this
#[aoc(day9, part2)]
pub fn part2(input: &Input) -> u64 {
    let mut files = Vec::new();
    files.push(Block {
        size: input[0] as u64,
        id: Some(0),
    });
    for (id, chunk) in input[1..].chunks(2).enumerate() {
        let &[empty, full] = chunk else {
            unreachable!()
        };
        files.push(Block {
            size: empty as _,
            id: None,
        });
        files.push(Block {
            size: full as _,
            id: Some(id as u64 + 1),
        });
    }

    let mut back_ptr = input.len() - 1;
    while back_ptr > 0 {
        if files[back_ptr].id.is_none() {
            back_ptr -= 1;
            continue;
        }
        let size = files[back_ptr].size;
        let mut empty_slots = files
            .iter()
            .enumerate()
            .filter(|(_, block)| block.id.is_none());
        let Some((mut front_ptr, _)) = empty_slots.next() else {
            break;
        };
        while front_ptr < back_ptr {
            match files[front_ptr].size.cmp(&size) {
                std::cmp::Ordering::Equal => {
                    files.swap(front_ptr, back_ptr);
                    break;
                }
                std::cmp::Ordering::Greater => {
                    files[front_ptr].size -= size;
                    files.insert(front_ptr, files[back_ptr]);
                    back_ptr += 1;
                    files[back_ptr].id = None;
                    break;
                }
                _ => {}
            }

            let Some((next_empty, _)) = empty_slots.next() else {
                break;
            };
            front_ptr = next_empty;
        }
        back_ptr -= 1;
    }

    let mut index = 0;
    let mut res = 0;
    for block in files {
        if let Some(id) = block.id {
            res += id * triangle_sum(index, index + block.size);
        }
        index += block.size;
    }

    res
}
