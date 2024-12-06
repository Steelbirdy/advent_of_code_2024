use crate::{HashMap, HashSet};

type Rules = HashMap<u32, HashSet<u32>>;

#[aoc_generator(day5)]
pub fn generator(input: &str) -> (Rules, Vec<Vec<u32>>) {
    let mut lines = input.lines();

    let mut rules = Rules::default();
    for line in lines.by_ref().take_while(|line| !line.is_empty()) {
        let (a, b) = line.split_once('|').unwrap();
        let a = a.parse().unwrap();
        let b = b.parse().unwrap();
        rules.entry(a).or_default().insert(b);
    }

    let updates = lines
        .map(|line| line.split(',').map(|x| x.parse().unwrap()).collect())
        .collect();

    (rules, updates)
}

#[aoc(day5, part1)]
pub fn part1((rules, updates): &(Rules, Vec<Vec<u32>>)) -> u32 {
    let mut result = 0;
    'outer: for update in updates {
        for (i, lhs) in update.iter().enumerate() {
            let lhs_rules = &rules[lhs];
            if update[..i].iter().any(|rhs| lhs_rules.contains(rhs)) {
                continue 'outer;
            }
        }
        result += update[update.len() / 2];
    }
    result
}

struct Part2<'a> {
    rules: &'a Rules,
    update_set: HashSet<u32>,
    no_incoming: HashSet<u32>,
    rules_subset: Rules,
    rules_rev_subset: Rules,
    sorted: Vec<u32>,
}

impl<'a> Part2<'a> {
    fn new(rules: &'a Rules) -> Self {
        Self {
            rules,
            update_set: HashSet::default(),
            no_incoming: HashSet::default(),
            rules_subset: Rules::default(),
            rules_rev_subset: Rules::default(),
            sorted: Vec::new(),
        }
    }

    // Does a topological sort on the first half of the list. Only uses relevant rules (rules (a,b) where a and b are both in the list)
    fn run(&mut self, update: &[u32]) -> u32 {
        let Self {
            rules,
            update_set,
            no_incoming,
            rules_subset,
            rules_rev_subset,
            sorted,
        } = self;

        update_set.extend(update.iter().copied());
        no_incoming.extend(update.iter().copied());

        for &x in update {
            let rules_x = rules_subset.entry(x).or_default();
            rules_x.extend(rules[&x].intersection(update_set));
            for &y in rules_x.iter() {
                rules_rev_subset.entry(y).or_default().insert(x);
                no_incoming.remove(&y);
            }
        }

        while sorted.len() <= update.len() / 2 {
            let &n = no_incoming.iter().next().unwrap();
            no_incoming.remove(&n);
            sorted.push(n);
            for &m in &rules_subset[&n] {
                let rev = rules_rev_subset.get_mut(&m).unwrap();
                rev.remove(&n);
                if rev.is_empty() {
                    no_incoming.insert(m);
                }
            }
        }

        let ret = sorted[sorted.len() - 1];
        self.clear();
        ret
    }

    fn clear(&mut self) {
        self.update_set.clear();
        self.no_incoming.clear();
        self.rules_subset.clear();
        self.rules_rev_subset.clear();
        self.sorted.clear();
    }
}

#[aoc(day5, part2)]
pub fn part2((rules, updates): &(Rules, Vec<Vec<u32>>)) -> u32 {
    let mut result = 0;
    let mut runner = Part2::new(rules);

    for update in updates {
        let mut is_sorted = true;
        for (i, lhs) in update.iter().enumerate() {
            let lhs_rules = &rules[lhs];
            if update[..i].iter().any(|rhs| lhs_rules.contains(rhs)) {
                is_sorted = false;
                break;
            }
        }
        if is_sorted {
            continue;
        }

        result += runner.run(update);
    }

    result
}
