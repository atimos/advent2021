pub mod task1 {
    use std::str::FromStr;

    pub fn run() {
        let position = include_str!("../data/day2/task1_live.txt")
            .lines()
            .map(|line| {
                let (command, steps) = line.split_once(' ').unwrap();
                (command, usize::from_str(steps).unwrap())
            })
            .fold((0, 0), |mut pos, (command, step)| {
                match command {
                    "forward" => pos.0 += step,
                    "up" => pos.1 -= step,
                    "down" => pos.1 += step,
                    _ => unreachable!(),
                }
                pos
            });
        dbg!(position.0 * position.1);
    }
}

pub mod task2 {
    use std::str::FromStr;

    pub fn run() {
        let position = include_str!("../data/day2/task2_live.txt")
            .lines()
            .map(|line| {
                let (command, steps) = line.split_once(' ').unwrap();
                (command, usize::from_str(steps).unwrap())
            })
            .fold((0, 0, 0), |mut pos, (command, step)| {
                match command {
                    "up" => pos.2 -= step,
                    "down" => pos.2 += step,
                    "forward" => {
                        pos.0 += step;
                        pos.1 += step * pos.2;
                    }
                    _ => unreachable!(),
                }
                pos
            });
        dbg!(position.0 * position.1);
    }
}
