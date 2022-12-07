pub fn solve(input: String) -> u32 {
    input
        .split("\n\n")
        .map(|inv| inv.lines().map(|food| food.parse::<u32>().unwrap()).sum())
        .max()
        .unwrap()
}
