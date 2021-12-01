pub mod task1 {
    use itertools::Itertools;
    use std::str::FromStr;

    pub fn run() {
        let result: usize = include_str!("../data/day1/task1_live.txt")
            .lines()
            .map(usize::from_str)
            .map(Result::unwrap)
            .tuple_windows()
            .map(|(prev, next)| (next > prev) as usize)
            .sum();
        dbg!(result);
    }
}

pub mod task2 {
    use itertools::Itertools;
    use std::str::FromStr;

    pub fn run() {
        let result: usize = include_str!("../data/day1/task2_live.txt")
            .lines()
            .map(usize::from_str)
            .map(Result::unwrap)
            .tuple_windows()
            .map(|(first, second, third)| first + second + third)
            .tuple_windows()
            .map(|(prev, next)| (next > prev) as usize)
            .sum();
        dbg!(result);
    }
}
