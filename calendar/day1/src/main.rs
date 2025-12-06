use day1::analyze_password;

fn main() {
    println!("Starting...");
    solve();
}

fn solve() {
    let input = include_str!("input");
    let result = analyze_password(input);
    println!("Solution: {result}");
}
