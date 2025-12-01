use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn calculate_dial0(path: impl AsRef<Path>) -> usize {
    let mut number = 50;
    BufReader::new(File::open(path).unwrap())
        .lines()
        .map(Result::unwrap)
        .filter(|line| !line.trim().is_empty())
        .filter(|line| {
            let direction = line.chars().next().unwrap();
            let value = line[1..].parse::<u16>().unwrap() % 100;
            number = if direction == 'L' {
                if number < value {
                    100 - value + number
                } else {
                    number - value
                }
            } else {
                (number + value) % 100
            };
            number == 0
        })
        .count()
}

fn main() {
    println!("{}", calculate_dial0("data.txt"));
}

#[cfg(test)]
mod tests {

    use crate::calculate_dial0;

    #[test]
    fn test1() {
        assert_eq!(calculate_dial0("test.txt"), 3);
    }
}
