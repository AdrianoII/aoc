fn parse_range(range: &str) -> (u32, u32) {
    let mut bounds = range.split('-').map(|bound| bound.parse::<u32>().unwrap());
    (bounds.next().unwrap(), bounds.next().unwrap())
}

pub fn solve(input: String) -> u32 {
    input
        .lines()
        .filter(|&ranges| {
            let mut ranges = ranges.split(',');
            let (lb1, hb1) = parse_range(ranges.next().unwrap());
            let (lb2, hb2) = parse_range(ranges.next().unwrap());
            (lb1 <= lb2 && hb1 >= lb2) || (lb2 <= lb1 && hb2 >= lb1)
        })
        .count() as u32
}
