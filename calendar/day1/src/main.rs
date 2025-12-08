use day1;

fn main() {
    println!("Starting...");
    solve();
    solve_part_two();
}

fn solve() {
    println!("Solving Part One");
    let input = include_str!("input");
    let result = day1::analyze_password(input);
    println!("Solution: {result}");
}

fn solve_part_two() {
    println!("Solving Part Two");
    let input = include_str!("input");
    let result = day1::analyze_password_0x434c49434b(input);
    println!("Solution: {result}");
}
