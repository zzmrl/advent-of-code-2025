fn main() {
    let input = include_str!("input");
    println!("Testing part one");
    let result = part_one(input);
    println!("Result: {result}");
    // println!("Testing part two");
    // let result2 = part_two(input);
    // println!("Result: {result2}");
}

fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut highest = String::from("");
            for (i, c1) in line.chars().enumerate() {
                for c2 in line.chars().skip(i + 1) {
                    let num = format!("{c1}{c2}");
                    if num > highest {
                        highest = num;
                    }
                }
            }
            highest.parse::<u32>().unwrap()
        })
        .sum()
}

fn part_two(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(EXAMPLE_INPUT), 357);
    }

    // #[test]
    // fn test_part_two() {
    //     assert_eq!(part_two(EXAMPLE_INPUT), 2);
    // }
}
