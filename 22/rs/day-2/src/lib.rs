pub mod tasks;

#[cfg(test)]
mod tests {
    pub use crate::tasks::{task_1, task_2};
    use std::fs;

    #[test]
    fn task_1_example() {
        let path = "../../inputs/2-ex.in";
        let input = fs::read_to_string(path).unwrap();
        assert_eq!(task_1::solve(input), 15);
    }

    #[test]
    fn task_1() {
        let path = "../../inputs/2.in";
        let input = fs::read_to_string(path).unwrap();
        assert_eq!(task_1::solve(input), 15422);
    }

    #[test]
    fn task_2_example() {
        let path = "../../inputs/2-ex.in";
        let input = fs::read_to_string(path).unwrap();
        assert_eq!(task_2::solve(input), 12);
    }

    #[test]
    fn task_2() {
        let path = "../../inputs/2.in";
        let input = fs::read_to_string(path).unwrap();
        assert_eq!(task_2::solve(input), 15442);
    }
}
