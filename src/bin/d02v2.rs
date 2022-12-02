fn main() {
    println!(
        "{}",
        include_str!("day02.txt")
            .lines()
            .map(|x| match x {
                "A X" => 3, // 3 + 0
                "A Y" => 4, // 1 + 3
                "A Z" => 8, // 2 + 6
                "B X" => 1, // 1 + 0
                "B Y" => 5, // 2 + 3
                "B Z" => 9, // 3 + 6
                "C X" => 2, // 2 + 0
                "C Y" => 6, // 3 + 3
                "C Z" => 7, // 1 + 6
                _ => panic!("We are fooled!"),
            })
            .sum::<u64>()
    );
}
