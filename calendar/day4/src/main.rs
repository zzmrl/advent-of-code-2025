fn main() {
    let input = include_str!("input");
    println!("Testing part one");
    let result = part_one(input);
    println!("Result: {result}");
    println!("Testing part two");
    let result2 = part_two(input);
    println!("Result: {result2}");
}

fn part_one(input: &str) -> u32 {
    !todo!("Implement part one")
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
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(EXAMPLE_INPUT);
        assert_eq!(result, 0);
    }
}
