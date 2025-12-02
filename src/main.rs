use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

/// Counts the number of times the dial hits 0
fn calculate_dial0(path: impl AsRef<Path>) -> u16 {
    // Initialize the current dial position (number), file buffer, line buffer, and zero count
    let mut number = 50;
    let mut file = BufReader::new(File::open(path).unwrap());
    let mut line = String::new();
    let mut zeros = 0;

    // Iterate through the lines
    while let Ok(bytes_read) = file.read_line(&mut line) {
        // Stop reading if nothing was read
        if bytes_read == 0 {
            break;
        }

        // Take the first char to determine the rotation direction
        let direction = line.as_bytes()[0];

        // Read the other bytes of the line to determine how much to rotate by
        let mut rotation = line[1..].trim().parse::<u16>().unwrap();

        // Increase the number of zeros by the number of full rotations and limit the amount to
        // rotate by to an amount below 100.
        zeros += rotation / 100;
        rotation %= 100;

        // Rotate and increment the number of zeros if needed
        number = if direction == b'L' {
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

        // Clear the buffer
        line.clear();
    }

    zeros
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
