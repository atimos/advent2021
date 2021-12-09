pub mod task1 {
    pub fn run() {
        let rates = include_str!("../data/day3/task1_live.txt")
            .lines()
            .map(|line| line.chars().map(|char| f64::from(char.to_digit(2).unwrap())).collect())
            .collect::<Vec<Vec<f64>>>();

        let rates = rates.iter().fold(vec![0.; rates[0].len()], |mut sum, rates| {
            rates.iter().enumerate().for_each(|(idx, value)| {
                sum[idx] += value;
            });
            sum
        });

        let gamma = rates
            .iter()
            .map(|sum| {
                #[allow(clippy::cast_precision_loss)]
                (((*sum / rates.len() as f64) >= 0.5) as usize).to_string()
            })
            .collect::<String>();
        let gamma = i32::from_str_radix(&gamma, 2).unwrap();

        let epsilon = rates
            .iter()
            .map(|sum| {
                #[allow(clippy::cast_precision_loss)]
                (((*sum / rates.len() as f64) <= 0.5) as usize).to_string()
            })
            .collect::<String>();
        let epsilon = i32::from_str_radix(&epsilon, 2).unwrap();

        dbg!(gamma * epsilon);
    }
}

pub mod task2 {
    pub fn run() {
        let rates = include_str!("../data/day3/task1_live.txt")
            .lines()
            .map(|line| line.chars().map(|char| f64::from(char.to_digit(2).unwrap())).collect())
            .collect::<Vec<Vec<f64>>>();

        let generator = i32::from_str_radix(
            &filter(rates.clone(), 0, |max, min| if max.len() > min.len() { max } else { min })
                .iter()
                .map(ToString::to_string)
                .collect::<String>(),
            2,
        )
        .unwrap();

        let scrubber = i32::from_str_radix(
            &filter(rates, 0, |max, min| if max.len() > min.len() { min } else { max })
                .iter()
                .map(ToString::to_string)
                .collect::<String>(),
            2,
        )
        .unwrap();

        dbg!(generator * scrubber);
    }

    fn filter(
        list: Vec<Vec<f64>>,
        idx: usize,
        cb: impl Fn(Vec<Vec<f64>>, Vec<Vec<f64>>) -> Vec<Vec<f64>>,
    ) -> Vec<f64> {
        if list.len() == 1 {
            return list[0].clone();
        }

        let split = list.into_iter().fold(
            (Vec::default(), Vec::default()),
            |mut split: (Vec<Vec<f64>>, Vec<Vec<f64>>), rate| {
                if rate[idx] == 0. {
                    split.0.push(rate);
                } else {
                    split.1.push(rate);
                }
                split
            },
        );

        filter(cb(split.0, split.1), idx + 1, cb)
    }
}
