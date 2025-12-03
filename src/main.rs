use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn sum_max_jolt(path: impl AsRef<Path>) -> u64 {
    // Read the data file
    BufReader::new(File::open(path).unwrap())
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            let digits = line
                .trim()
                .bytes()
                .map(|byte| byte - b'0')
                .collect::<Vec<_>>();
            let digits = digits.as_slice();
            u64::from(
                (0..digits.len())
                    .flat_map(|first| {
                        (first + 1..digits.len())
                            .map(move |second| digits[first] * 10 + digits[second])
                    })
                    .max()
                    .unwrap(),
            )
        })
        .sum()
}

fn main() {
    println!("{}", sum_max_jolt("data.txt"));
}

#[cfg(test)]
mod tests {
    use crate::sum_max_jolt;

    #[test]
    fn test() {
        assert_eq!(sum_max_jolt("test.txt"), 357)
    }
}
