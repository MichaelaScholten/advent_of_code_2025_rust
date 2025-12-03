use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn invalid_id(id: &str) -> bool {
    id.starts_with('0') || id[..id.len() / 2] == id[id.len() / 2..]
}

// Too high
fn sum_invalid_ids(path: impl AsRef<Path>) -> u64 {
    BufReader::new(File::open(path).unwrap())
        .split(b',')
        .map(Result::unwrap)
        .map(|range| {
            let mut parts = range.split(|&byte| byte == b'-');
            let first = str::from_utf8(parts.next().unwrap()).unwrap();
            let second = str::from_utf8(parts.next().unwrap()).unwrap();
            assert!(parts.next().is_none());

            let first_int = first.trim().parse::<u64>().unwrap();
            let second_int = second.trim().parse().unwrap();
            let mut bad_sum = 0;
            if invalid_id(first) {
                bad_sum += first_int;
            }
            if invalid_id(second) {
                bad_sum += second_int;
            }
            bad_sum
                + (first_int..second_int)
                    .skip(1)
                    .filter(|&number| invalid_id(&number.to_string()))
                    .sum::<u64>()
        })
        .sum()
}

fn main() {
    println!("{}", sum_invalid_ids("data.txt"));
}

#[cfg(test)]
mod tests {
    use crate::sum_invalid_ids;

    #[test]
    fn test() {
        assert_eq!(sum_invalid_ids("test.txt"), 1227775554);
    }
}
