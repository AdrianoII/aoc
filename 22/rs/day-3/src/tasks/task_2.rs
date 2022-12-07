use itertools::Itertools;
use std::collections::BTreeSet;

pub fn solve(input: String) -> u32 {
    input
        .lines()
        .tuples::<(_, _, _)>()
        .map(|(b1, b2, b3)| {
            let b1 = BTreeSet::from_iter(b1.chars());
            let b2 = BTreeSet::from_iter(b2.chars());
            let b3 = BTreeSet::from_iter(b3.chars());
            match (&(&b1 & &b2) & &b3).iter().next().unwrap() {
                c @ 'a'..='z' => *c as u32 - 96,
                c @ 'A'..='Z' => *c as u32 - 38,
                _ => panic!(""),
            }
        })
        .sum()
}
