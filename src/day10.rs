use std::fmt::{Debug, Display, Formatter, Write};

const INPUT: &str = include_str!("../inputs/day10.txt");

enum Command {
    Addx(isize),
    Noop,
}

impl Command {
    fn from_str(s: &str) -> Self {
        match s.as_bytes()[0] {
            b'a' => Command::Addx(s[5..].parse().unwrap()),
            b'n' => Command::Noop,
            _ => unreachable!(),
        }
    }

    const fn cycle_count(&self) -> usize {
        match self {
            Command::Addx(_) => 2,
            Command::Noop => 1,
        }
    }

    fn apply(&self, x: &mut isize) {
        match self {
            Command::Addx(n) => *x += n,
            Command::Noop => (),
        }
    }
}

pub fn part1() -> isize {
    let mut x: isize = 1;
    let mut cycle: usize = 0;
    let mut target_cycles = [20_usize, 60, 100, 140, 180, 220].iter().peekable();
    let mut total_signal_strength: isize = 0;
    let commands = INPUT.lines().map(Command::from_str);
    for c in commands {
        cycle += c.cycle_count();
        if let Some(&&target_cycle) = target_cycles.peek() {
            if cycle >= target_cycle {
                let _ = target_cycles.next();
                total_signal_strength += x * (target_cycle as isize);
            }
        } else {
            // no more target cycles, no need to finish the loop
            break;
        }
        c.apply(&mut x)
    }
    total_signal_strength
}

const LINE_WIDTH: usize = 40;
const LINES: usize = 6;

pub struct CRTScreen {
    lines: [[u8; LINE_WIDTH]; LINES],
}

impl CRTScreen {
    const fn new() -> Self {
        Self {
            lines: [[b'.'; LINE_WIDTH]; LINES],
        }
    }
}

impl Display for CRTScreen {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "CRTScreen:")?;
        self.lines.iter().for_each(|l| {
            l.iter().for_each(|c| {
                f.write_char(*c as char).unwrap();
            });
            writeln!(f).unwrap();
        });
        Ok(())
    }
}

impl Debug for CRTScreen {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

pub fn part2() -> CRTScreen {
    let mut screen = CRTScreen::new();

    let mut sprite_center: isize = 1;
    let mut cycle: usize = 0;
    INPUT.lines().map(Command::from_str).for_each(|c| {
        let cycle_count = c.cycle_count();
        for _ in 0..cycle_count {
            let (line_index, line_x) = (cycle / LINE_WIDTH, cycle % LINE_WIDTH);
            let line = &mut screen.lines[line_index];
            if (sprite_center - 1..=sprite_center + 1).contains(&(line_x as isize)) {
                line[line_x] = b'#';
            }
            cycle += 1;
        }

        c.apply(&mut sprite_center)
    });

    screen
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate test;

    use test::Bencher;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 13480);
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(part1);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2().to_string(),
            // the letters are 'EGJBGCFK'
            r#"CRTScreen:
####..##....##.###...##...##..####.#..#.
#....#..#....#.#..#.#..#.#..#.#....#.#..
###..#.......#.###..#....#....###..##...
#....#.##....#.#..#.#.##.#....#....#.#..
#....#..#.#..#.#..#.#..#.#..#.#....#.#..
####..###..##..###...###..##..#....#..#.
"#
        );
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(part2);
    }
}
