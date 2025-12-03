use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn invalid_id(id: &str) -> bool {
    // If the id starts with 0 or the digits repeat each other (individually or as group)
    // it's invalid.
    id.starts_with('0')
        || (2..=id.len()).any(|splits| {
            if !id.len().is_multiple_of(splits) {
                return false;
            }
            let part_size = id.len() / splits;
            (1..splits).all(|part| id[..part_size] == id[part * part_size..(part + 1) * part_size])
        })
}

fn sum_invalid_ids(path: impl AsRef<Path>) -> u64 {
    // Read the data file
    BufReader::new(File::open(path).unwrap())
        // Split on ranges and panic on errrors
        .split(b',')
        .map(Result::unwrap)
        .map(|range| {
            // Split the range in 2
            let mut parts = range.split(|&byte| byte == b'-');

            // Turn both parts into strings
            let first = str::from_utf8(parts.next().unwrap()).unwrap();
            let second = str::from_utf8(parts.next().unwrap()).unwrap();

            // Panic if the range contains more than 2 values
            assert!(parts.next().is_none());

            // Parse both parts of the range as integers
            let first_int = first.trim().parse::<u64>().unwrap();
            let second_int = second.trim().parse().unwrap();

            // Check for invalid id's in the range and sum them
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
        assert_eq!(sum_invalid_ids("test.txt"), 4174379265);
    }
}
