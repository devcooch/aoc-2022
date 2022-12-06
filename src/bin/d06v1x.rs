fn main() {
    println!(
        "{}",
        include_str!("day06.txt")
            .lines()
            .next()
            .unwrap()
            .chars()
            .map(|c| (c as usize) - ('a' as usize))
            .collect::<Vec<_>>()
            .windows(4)
            .map(|window| window.iter().fold(0u32, |acc, x| acc | 1 << x))
            .take_while(|mask| mask.count_ones() < 4)
            .count()
            + 4
    );
}
