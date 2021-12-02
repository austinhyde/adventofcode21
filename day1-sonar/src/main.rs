
fn main() {
    println!("Part 1 answer: {}", part1(include_str!("part1.txt")));
    // answer: 1342

    println!("Part 2 answer: {}", part2(include_str!("part1.txt")));
    // answer: 1378
}

fn part1(input: &str) -> usize {
    count_increases(
        input.lines()
        .filter_map(|l| l.parse().ok())
        .collect()
    )
}

fn part2(input: &str) -> usize {
    count_increases(
        input.lines()
            .filter_map(|l| l.parse().ok())
            .collect::<Vec<usize>>()
            .windows(3)
            .map(|w| w.iter().sum())
            .collect()
    )
}

fn count_increases(v: Vec<usize>) -> usize {
    v.windows(2)
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

    #[test]
    fn example2() {
        let input = include_str!("example1.txt");
        assert_eq!(part2(input), 5);
    }
}