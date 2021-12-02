
fn main() {
    println!("Part 1 answer: {}", part1(include_str!("part1.txt")));
    // answer: 1342
}

fn part1(input: &str) -> usize {
    input.lines()
        .filter_map(|l| l.parse().ok())
        .collect::<Vec<usize>>()
        .windows(2)
        .filter(|w| w[1] > w[0])
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = include_str!("example1.txt");
        assert_eq!(part1(input), 7)
    }
}