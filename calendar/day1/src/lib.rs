pub mod dial;

fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| {
            let (direction, distance) = line.split_at(1);
            let distance: i32 = distance.parse().unwrap();
            match direction {
                "L" => -distance,
                "R" => distance,
                _ => panic!("Invalid direction"),
            }
        })
        .collect()
}

pub fn rotate_left(pos: u32, steps: u32) -> u32 {
    100 + pos - steps % 100
}

// fn count_zeroes(input: &str) -> i32 {
//     let mut pos = 0u8;

//     for line in input.trim().lines() {
//         let (direction, distance) = line.split_at(1);
//         let distance: i32 = distance.parse().unwrap();
//         match direction {
//             "L" => -distance,
//             "R" => distance,
//             _ => panic!("Invalid direction"),
//         }
//     }
//     0
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_left() {
        assert_eq!(rotate_left(50, 68), 82)
    }
}
