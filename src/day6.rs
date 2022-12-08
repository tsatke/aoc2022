const INPUT: &str = include_str!("../inputs/day6.txt");

pub fn part1() -> usize {
    // Although both parts could be solved with a bool array for the `seen` letters,
    // this approach for part 1 only requires 65% of the time.

    const WINDOW_SIZE: usize = 4;
    INPUT
        .as_bytes()
        .windows(WINDOW_SIZE)
        .position(|s| {
            (s[0] != s[1] && s[0] != s[2] && s[0] != s[3])
                && (s[1] != s[2] && s[1] != s[3])
                && (s[2] != s[3])
        })
        .unwrap()
        + WINDOW_SIZE
}

pub fn part2() -> usize {
    // Unlike for part 1, using explicit comparisons here makes the whole thing
    // a lot slower (by factors).

    const WINDOW_SIZE: usize = 14;
    INPUT
        .as_bytes()
        .windows(WINDOW_SIZE)
        .position(|s| {
            let mut seen = [false; 26]; // let's hope it's only a-z
            for &e in s {
                let c = (e - b'a') as usize;
                if seen[c] {
                    return false;
                }
                seen[c] = true;
            }
            true
        })
        .unwrap()
        + WINDOW_SIZE
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate test;

    use test::Bencher;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 1702);
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(part1);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 3559);
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(part2);
    }
}
