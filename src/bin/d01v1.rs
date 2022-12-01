use itertools::Itertools;

fn main() {
    println!(
        "{}",
        include_str!("day01.txt")
            .lines()
            .group_by(|line| line.is_empty())
            .into_iter()
            .map(|(_, group)| group
                .map(|l| l.parse::<usize>().unwrap_or_default())
                .sum::<usize>())
            .max()
            .unwrap()
    );
}
