use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn calculate_dial0(path: impl AsRef<Path>) -> u16 {
    let mut number = 50;
    BufReader::new(File::open(path).unwrap())
        .lines()
        .map(Result::unwrap)
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let direction = line.chars().next().unwrap();
            let mut rotation = line[1..].parse::<u16>().unwrap();
            let mut zeros = rotation / 100;
            rotation %= 100;
            number = if direction == 'L' {
                if number < rotation {
                    if number != 0 {
                        zeros += 1;
                    }
                    100 - rotation + number
                } else {
                    number - rotation
                }
            } else {
                if number + rotation > 100 {
                    zeros += 1;
                }
                (number + rotation) % 100
            };
            if number == 0 {
                zeros += 1;
            }
            zeros
        })
        .sum()
}

fn main() {
    println!("{}", calculate_dial0("data.txt"));
}

#[cfg(test)]
mod tests {

    use crate::calculate_dial0;

    #[test]
    fn test1() {
        assert_eq!(calculate_dial0("test.txt"), 6);
    }
}
