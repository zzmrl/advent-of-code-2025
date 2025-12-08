pub fn analyze_password(input: &str) -> i16 {
    input
        .lines()
        .fold((50, 0), |(pos, count), line| {
            let (direction, distance) = line.split_at(1);
            let mut distance: i16 = distance.parse().unwrap();
            if direction == "L" {
                distance *= -1;
            }
            let pos = (pos + distance).rem_euclid(100);
            (pos, count + (pos == 0) as i16)
        })
        .1
}

pub fn analyze_password_0x434c49434b(input: &str) -> i16 {
    input
        .lines()
        .fold((50, 0), |(pos, count), line| {
            let (direction, distance) = line.split_at(1);
            let mut distance: i16 = distance.parse().unwrap();
            if direction == "L" {
                distance *= -1;
            }
            let new_pos = pos + distance;
            (
                new_pos.rem_euclid(100),
                count + (new_pos <= 0 && pos != 0) as i16 + (new_pos.signum() * new_pos / 100),
            )
        })
        .1
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";

    #[test]
    fn test_analyze_password() {
        assert_eq!(analyze_password(EXAMPLE_INPUT), 3);
    }

    #[test]
    fn test_analyze_password_0x434c49434b() {
        assert_eq!(analyze_password_0x434c49434b(EXAMPLE_INPUT), 6);
    }
}
