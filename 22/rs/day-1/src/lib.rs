pub mod tasks;

#[cfg(test)]
mod tests {
    pub use crate::tasks::{task_1, task_2};
    use std::fs;

    #[test]
    fn task_1_example() {
        let path = "../../inputs/1-ex.in";
        let input = fs::read_to_string(path).unwrap();
        assert_eq!(task_1::solve(input), 24000);
    }

    #[test]
    fn task_1() {
        let path = "../../inputs/1.in";
        let input = fs::read_to_string(path).unwrap();
        assert_eq!(task_1::solve(input), 66616);
    }

    #[test]
    fn task_2_example() {
        let path = "../../inputs/1-ex.in";
        let input = fs::read_to_string(path).unwrap();
        assert_eq!(task_2::solve(input), 45000);
    }

    #[test]
    fn task_2() {
        let path = "../../inputs/1.in";
        let input = fs::read_to_string(path).unwrap();
        assert_eq!(task_2::solve(input), 199172);
    }
}
