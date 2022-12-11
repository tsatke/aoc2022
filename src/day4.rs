const INPUT: &str = include_str!("../inputs/day4.txt");

fn ranges() -> impl Iterator<Item = ((usize, usize), (usize, usize))> {
    INPUT.lines().map(|l| {
        let mut split = l.split(|c| c == ',' || c == '-');
        let left_start = split.next().unwrap().parse::<usize>().unwrap();
        let left_end = split.next().unwrap().parse::<usize>().unwrap();
        let right_start = split.next().unwrap().parse::<usize>().unwrap();
        let right_end = split.next().unwrap().parse::<usize>().unwrap();
        ((left_start, left_end), (right_start, right_end))
    })
}

pub fn part1() -> usize {
    ranges()
        .filter(|(left, right)| {
            (left.0 <= right.0 && left.1 >= right.0 && left.0 <= right.1 && left.1 >= right.1)
                || (right.0 <= left.0
                    && right.1 >= left.0
                    && right.0 <= left.1
                    && right.1 >= left.1)
        })
        .count()
}

pub fn part2() -> usize {
    ranges()
        .filter(|(left, right)| {
            left.0 <= right.0 && left.1 >= right.0
                || left.0 <= right.1 && left.1 >= right.1
                || right.0 <= left.0 && right.1 >= left.0
                || right.0 <= left.1 && right.1 >= left.1
        })
        .count()
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
