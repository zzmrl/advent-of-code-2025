pub fn rotate_left(pos: u32, steps: u32) -> u32 {
    let steps = steps % 100;
    if steps > pos {
        100 - (steps - pos)
    } else {
        pos - steps
    }
}

pub fn rotate_right(pos: u32, steps: u32) -> u32 {
    let steps = steps % 100;
    if steps >= 100 - pos {
        steps - (100 - pos)
    } else {
        pos + steps
    }
}

pub fn analyze_password(input: &str) -> u32 {
    let mut zero_count = 0u32;
    let mut pos = 50;
    for line in input.lines() {
        let (direction, distance) = line.split_at(1);
        let distance: u32 = distance.parse().unwrap();
        match direction {
            "L" => pos = rotate_left(pos, distance),
            "R" => pos = rotate_right(pos, distance),
            _ => panic!("Invalid direction"),
        }
        if pos == 0 {
            zero_count += 1;
        }
    }
    zero_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";

    #[test]
    fn test_rotate_left() {
        assert_eq!(rotate_left(50, 68), 82);
        assert_eq!(rotate_left(82, 30), 52);
        assert_eq!(rotate_left(0, 5), 95);
        assert_eq!(rotate_left(55, 55), 0);
    }

    #[test]
    fn test_rotate_right() {
        assert_eq!(rotate_right(52, 48), 0);
        assert_eq!(rotate_right(95, 60), 55);
        assert_eq!(rotate_right(0, 14), 14);
    }

    #[test]
    fn test_analyze_password() {
        assert_eq!(analyze_password(EXAMPLE_INPUT), 3);
    }
}
