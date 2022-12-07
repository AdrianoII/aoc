use std::collections::VecDeque;

fn parse_stack(stack_lines: &str) -> Vec<VecDeque<char>> {
    let mut lines = stack_lines.lines().collect::<Vec<&str>>();
    lines.pop();
    (1..lines.first().unwrap().len())
        .step_by(4)
        .map(|i| {
            lines
                .iter()
                .map(|&line| line.chars().nth(i).unwrap())
                .filter(|c| c.is_alphabetic())
                .collect::<VecDeque<char>>()
        })
        .collect::<Vec<VecDeque<char>>>()
}

fn parse_moves(moves_lines: &str) -> Vec<(usize, usize, usize)> {
    moves_lines
        .lines()
        .map(|l| {
            let mut words = l.split(' ');
            (
                words.nth(1).unwrap().parse::<usize>().unwrap(),
                words.nth(1).unwrap().parse::<usize>().unwrap(),
                words.nth(1).unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<(usize, usize, usize)>>()
}

pub fn solve(input: String) -> String {
    let mut inp_split = input.split("\n\n");
    let (stack_lines, moves_lines) = (inp_split.next().unwrap(), inp_split.next().unwrap());
    let mut stack = parse_stack(stack_lines);
    let moves = parse_moves(moves_lines);

    moves.into_iter().for_each(|(n, from, to)| {
        for _ in 0..n {
            let e = stack.get_mut(from - 1).unwrap().pop_front().unwrap();
            stack.get_mut(to - 1).unwrap().push_front(e);
        }
    });

    stack
        .iter()
        .map(|col| col.front().unwrap())
        .collect::<String>()
}
