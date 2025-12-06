fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    const EXAMPLE_INPUT: &str = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";

    #[test]
    fn test_main() {
        assert_eq!(1, 1);
    }
}
