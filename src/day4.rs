use std::ops::RangeInclusive;

const INPUT: &str = include_str!("../inputs/day4.txt");

fn ranges() -> impl Iterator<Item = (RangeInclusive<isize>, RangeInclusive<isize>)> {
    INPUT.lines().map(|l| {
        let mut split = l.split(|c| c == ',' || c == '-');
        let left_start = split.next().unwrap().parse::<isize>().unwrap();
        let left_end = split.next().unwrap().parse::<isize>().unwrap();
        let right_start = split.next().unwrap().parse::<isize>().unwrap();
        let right_end = split.next().unwrap().parse::<isize>().unwrap();
        (left_start..=left_end, right_start..=right_end)
    })
}

pub fn part1() -> isize {
    ranges()
        .filter(|(left, right)| {
            (left.contains(right.start()) && left.contains(right.end()))
                || (right.contains(left.start()) && right.contains(left.end()))
        })
        .count() as isize
}

pub fn part2() -> isize {
    ranges()
        .filter(|(left, right)| {
            left.contains(right.start())
                || left.contains(right.end())
                || right.contains(left.start())
                || right.contains(left.end())
        })
        .count() as isize
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate test;

    use test::Bencher;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 528);
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(part1);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 881);
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(part2);
    }
}
