fn evaluate_match(match_str: &str) -> u32 {
    match match_str {
        "A X" => 3 + 1,
        "A Y" => 6 + 2,
        "A Z" => 0 + 3,
        "B X" => 0 + 1,
        "B Y" => 3 + 2,
        "B Z" => 6 + 3,
        "C X" => 6 + 1,
        "C Y" => 0 + 2,
        "C Z" => 3 + 3,
        a => panic!("Invalid match {:?}", a),
    }
}

pub fn solve(input: String) -> u32 {
    input.lines().map(evaluate_match).sum()
}
