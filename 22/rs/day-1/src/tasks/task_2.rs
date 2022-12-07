pub fn solve(input: String) -> u32 {
    let mut calories: Vec<u32> = input
        .split("\n\n")
        .map(|inv| inv.lines().map(|food| food.parse::<u32>().unwrap()).sum())
        .collect();
    calories.sort();
    calories.reverse();
    calories.into_iter().take(3).sum()
}
