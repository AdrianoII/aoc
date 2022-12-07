use std::collections::BTreeSet;

pub fn solve(input: String) -> u32 {
    input
        .lines()
        .map(|inv| inv.split_at(inv.len() / 2))
        .map(|(c1, c2)| {
            let c1 = BTreeSet::from_iter(c1.chars());
            let c2 = BTreeSet::from_iter(c2.chars());
            match (&c1 & &c2).iter().next().unwrap() {
                c @ 'a'..='z' => *c as u32 - 96,
                c @ 'A'..='Z' => *c as u32 - 38,
                _ => panic!(""),
            }
        })
        .sum()
}
