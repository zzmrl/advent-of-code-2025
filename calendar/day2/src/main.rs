fn main() {
    let input = include_str!("input");
    println!("Testing part one");
    let result = part_one(input);
    println!("Result: {result}");
}

fn part_one(input: &str) -> u64 {
    input
        .split(',')
        .map(|range| {
            let (start, end) = range.trim().split_once("-").unwrap();
            let start = start.parse::<u64>().unwrap();
            let end = end.parse::<u64>().unwrap();

            (start..=end)
                .map(|num| {
                    let strnum = num.to_string();
                    if strnum.len() % 2 == 0 {
                        let half = strnum.len() / 2;
                        if &strnum[..half] == &strnum[half..] {
                            return num;
                        }
                    }
                    0
                })
                .sum::<u64>()
        })
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
    1698522-1698528,446443-446449,38593856-38593862,565653-565659,
    824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 1227775554);
    }
}
