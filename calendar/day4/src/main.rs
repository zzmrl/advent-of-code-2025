use std::collections::HashMap;

fn main() {
    let input = include_str!("input");
    println!("Testing part one");
    let result = part_one(input);
    println!("Result: {result}");
    println!("Testing part two");
    let result2 = part_two(input);
    println!("Result: {result2}");
}

const DELTAS: [(i16, i16); 8] = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
    (-1, -1),
    (-1, 1),
    (1, -1),
    (1, 1),
];

fn part_one(input: &str) -> u32 {
    let map: HashMap<(i16, i16), u8> = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.bytes().enumerate().map(move |(col, v)| {
                let key = (row as i16, col as i16);
                let value = match v {
                    b'.' => 0,
                    b'@' => 1,
                    _ => panic!("Invalid input"),
                };
                (key, value)
            })
        })
        .collect();

    map.iter()
        .filter(|(_key, value)| **value == 1)
        .map(|((row, col), _value)| {
            let neighbors: u8 = DELTAS
                .iter()
                .map(|(dr, dc)| {
                    if let Some(&neighbor) = map.get(&(*row + dr, *col + dc)) {
                        neighbor
                    } else {
                        0
                    }
                })
                .sum();
            if neighbors < 4 { 1 } else { 0 }
        })
        .sum()
}

fn part_two(input: &str) -> u32 {
    !todo!("Implement part two")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part_one() {
        let result = part_one(EXAMPLE_INPUT);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(EXAMPLE_INPUT);
        assert_eq!(result, 0);
    }
}
