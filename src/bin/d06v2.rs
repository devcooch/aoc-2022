fn main() {
    println!(
        "{}",
        include_str!("day06.txt")
            .lines()
            .next()
            .unwrap()
            .chars()
            .map(|c| (c as usize) - ('a' as usize))
            .scan([0i32; 26], |state, x| {
                state.iter_mut().for_each(|i| *i -= 1);
                state[x] = 14;
                Some(*state)
            })
            .take_while(|state| state.iter().filter(|x| *x > &0).count() < 14)
            .count()
            + 1
    );
}
