pub mod tasks;

#[cfg(test)]
mod tests {
    pub use crate::tasks::{task_1, task_2};
    use std::fs;

    #[test]
    fn task_1_example_1() {
        let path = "../../inputs/6-ex-1.in";
        let input = fs::read_to_string(path).unwrap();
        assert_eq!(task_1::solve(input), 7);
    }

    #[test]
    fn task_1_example_2() {
        let path = "../../inputs/6-ex-2.in";
        let input = fs::read_to_string(path).unwrap();
        assert_eq!(task_1::solve(input), 5);
    }

    #[test]
    fn task_1_example_3() {
        let path = "../../inputs/6-ex-3.in";
        let input = fs::read_to_string(path).unwrap();
        assert_eq!(task_1::solve(input), 6);
    }

    #[test]
    fn task_1_example_4() {
        let path = "../../inputs/6-ex-4.in";
        let input = fs::read_to_string(path).unwrap();
        assert_eq!(task_1::solve(input), 10);
    }

    #[test]
    fn task_1_example_5() {
        let path = "../../inputs/6-ex-5.in";
        let input = fs::read_to_string(path).unwrap();
        assert_eq!(task_1::solve(input), 11);
    }

    #[test]
    fn task_1() {
        let path = "../../inputs/6.in";
        let input = fs::read_to_string(path).unwrap();
        assert_eq!(task_1::solve(input), 1987);
    }

    #[test]
    fn task_2_example_1() {
        let path = "../../inputs/6-ex-1.in";
        let input = fs::read_to_string(path).unwrap();
        assert_eq!(task_2::solve(input), 19);
    }

    #[test]
    fn task_2_example_2() {
        let path = "../../inputs/6-ex-2.in";
        let input = fs::read_to_string(path).unwrap();
        assert_eq!(task_2::solve(input), 23);
    }

    #[test]
    fn task_2_example_3() {
        let path = "../../inputs/6-ex-3.in";
        let input = fs::read_to_string(path).unwrap();
        assert_eq!(task_2::solve(input), 23);
    }

    #[test]
    fn task_2_example_4() {
        let path = "../../inputs/6-ex-4.in";
        let input = fs::read_to_string(path).unwrap();
        assert_eq!(task_2::solve(input), 29);
    }

    #[test]
    fn task_2_example_5() {
        let path = "../../inputs/6-ex-5.in";
        let input = fs::read_to_string(path).unwrap();
        assert_eq!(task_2::solve(input), 26);
    }

    #[test]
    fn task_2() {
        let path = "../../inputs/6.in";
        let input = fs::read_to_string(path).unwrap();
        assert_eq!(task_2::solve(input), 3059);
    }
}
