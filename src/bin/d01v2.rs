use itertools::Itertools;
use std::collections::BinaryHeap;

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
            .collect::<BinaryHeap<usize>>()
            .into_sorted_vec()
            .iter()
            .rev()
            .take(3)
            .sum::<usize>()
    );
}
