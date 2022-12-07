use std::collections::BTreeSet;

pub fn solve(input: String) -> usize {
    let input = input.lines().next().unwrap();
    input
        .as_bytes()
        .windows(4)
        .map(|w| w.iter().collect::<BTreeSet<&u8>>())
        .enumerate()
        .find(|(_, set)| set.len() == 4)
        .unwrap()
        .0
        + 4
}
