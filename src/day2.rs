const INPUT: &str = include_str!("../inputs/day2.txt");

pub fn part1() -> isize {
    INPUT
        .lines()
        .map(|l| {
            let b = l.as_bytes();
            (b[0], b[2])
        })
        .map(|(left, right)| {
            (match right {
                b'X' => 1_isize,
                b'Y' => 2_isize,
                b'Z' => 3_isize,
                _ => unreachable!(),
            }) + winner(left, right)
        })
        .sum()
}

fn winner(left: u8, right: u8) -> isize {
    match (left, right) {
        _ if left - b'A' == right - b'X' => 3,
        (b'A', b'Y') | (b'B', b'Z') | (b'C', b'X') => 6,
        _ => 0,
    }
}

pub fn part2() -> isize {
    INPUT
        .lines()
        .map(|l| {
            let b = l.as_bytes();
            (b[0], b[2])
        })
        .map(|(left, right)| {
            (match right {
                b'X' => 0_isize,
                b'Y' => 3_isize,
                b'Z' => 6_isize,
                _ => unreachable!(),
            }) + interpolate(left, right)
        })
        .sum()
}

fn interpolate(left: u8, outcome: u8) -> isize {
    match (left, outcome) {
        (left, b'Y') => match left {
            b'A' => 1,
            b'B' => 2,
            b'C' => 3,
            _ => unreachable!(),
        },
        (b'A', b'X') => 3, // lose
        (b'A', b'Z') => 2, // win
        (b'B', b'X') => 1,
        (b'B', b'Z') => 3,
        (b'C', b'X') => 2,
        (b'C', b'Z') => 1,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate test;
    use test::Bencher;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 12535);
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(part1);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 15457);
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(part2);
    }
}
