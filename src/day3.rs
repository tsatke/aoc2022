use std::ops::BitAnd;
use std::simd::u8x64;

const INPUT: &str = include_str!("../inputs/day3.txt");

pub fn part1() -> isize {
    INPUT
        .lines()
        .map(|l| (&l[0..l.len() / 2], &l[l.len() / 2..l.len()]))
        .map(|(left, right)| {
            let mut seen = [false; 52];
            let mut prio: isize = 0;
            left.bytes()
                .for_each(|b| seen[get_index(b) as usize] = true);
            for b in right.bytes() {
                let p = get_index(b);
                if seen[p as usize] {
                    prio += p as isize + 1; // p + 1 is the priority
                    seen[p as usize] = false;
                    break;
                }
            }
            prio
        })
        .sum()
}

fn get_index(b: u8) -> u8 {
    // this is essentially `if (b'a'..=b'z').contains(&b)` but faster
    let r = b.wrapping_sub(b'a');
    if r <= 26 {
        r
    } else {
        b - b'A' + 26
    }
}

pub fn part2() -> isize {
    let mut lines = INPUT.lines();
    let mut total: isize = 0;
    while let (Some(line1), Some(line2), Some(line3)) = (lines.next(), lines.next(), lines.next()) {
        let window = [line1, line2, line3];

        let mut seen = [[false; 52]; 4];
        for i in 0..window.len() {
            window[i]
                .bytes()
                .for_each(|b| seen[i][get_index(b) as usize] = true);
        }

        for i in 0..seen[3].len() {
            seen[3][i] = seen[0][i] && seen[1][i] && seen[2][i];
        }

        for (pos, &v) in seen[3].iter().enumerate() {
            if !v {
                continue;
            }
            total += pos as isize + 1;
        }
    }

    total
}

pub fn part2_simd() -> isize {
    let mut lines = INPUT.lines();
    let mut total: isize = 0;
    while let (Some(line1), Some(line2), Some(line3)) = (lines.next(), lines.next(), lines.next()) {
        let window = [line1, line2, line3];

        let mut seen = [
            u8x64::from_array([0_u8; 64]),
            u8x64::from_array([0_u8; 64]),
            u8x64::from_array([0_u8; 64]),
        ];
        for i in 0..window.len() {
            window[i]
                .bytes()
                .for_each(|b| seen[i][get_index(b) as usize] = 1);
        }

        let res = seen[0].bitand(seen[1]).bitand(seen[2]);

        for (pos, &v) in res.as_array().iter().take(52).enumerate() {
            if v == 0 {
                continue;
            }
            total += pos as isize + 1;
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate test;
    use test::Bencher;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 8233);
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(part1);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 2821);
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(part2);
    }

    #[test]
    fn test_part2_simd() {
        assert_eq!(part2_simd(), 2821);
    }

    #[bench]
    fn bench_part2_simd(b: &mut Bencher) {
        b.iter(part2_simd);
    }
}
