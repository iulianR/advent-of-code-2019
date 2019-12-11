use itertools::Itertools;

fn has_increasing_digits(x: &[u8]) -> bool {
    x.windows(2).all(|w| w[0] <= w[1])
}

fn has_5_distinct_digits(x: &[u8]) -> bool {
    x.windows(2).any(|x| x[0] == x[1])
}

fn has_one_or_more_double_digit(x: &[u8]) -> bool {
    x.iter()
        .group_by(|&n| n)
        .into_iter()
        .any(|(_, grp)| grp.count() == 2)
}

fn part1(start: i32, end: i32) -> usize {
    (start..=end)
        .map(|n| n.to_string().bytes().collect_vec())
        .filter(|x| has_increasing_digits(x) && has_5_distinct_digits(x))
        .count()
}

fn part2(start: i32, end: i32) -> usize {
    (start..=end)
        .map(|n| n.to_string().bytes().collect_vec())
        .filter(|x| has_increasing_digits(x) && has_one_or_more_double_digit(x))
        .count()
}

static INPUT: &str = include_str!("../input.txt");

fn main() {
    let mut range = INPUT.split('-').map(|n| n.trim().parse::<i32>().unwrap());
    let start = range.next().unwrap();
    let end = range.next().unwrap();
    println!("Part 1 {}", &part1(start, end));
    println!("Part 2 {}", &part2(start, end));
}
