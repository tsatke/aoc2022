const INPUT: &str = include_str!("../inputs/day9.txt");

#[derive(Debug, Clone)]
struct Coord(isize, isize);

impl Coord {
    fn x_mut(&mut self) -> &mut isize {
        &mut self.0
    }

    fn y_mut(&mut self) -> &mut isize {
        &mut self.1
    }

    fn x(&self) -> &isize {
        &self.0
    }

    fn y(&self) -> &isize {
        &self.1
    }

    fn is_adjacent_to(&self, other: &Self) -> bool {
        let dx = self.x() - other.x();
        let dy = self.y() - other.y();
        (-1..=1).contains(&dx) && (-1..=1).contains(&dy)
    }

    fn same_row_or_col(&self, other: &Self) -> bool {
        self.x() == other.x() || self.y() == other.y()
    }
}

fn parse_input() -> impl Iterator<Item = (u8, isize)> {
    INPUT
        .lines()
        .map(|l| (l.as_bytes()[0], &l[2..]))
        .map(|(c, n)| (c, n.parse::<isize>().unwrap()))
}

pub fn part1() -> usize {
    const NORMALIZE_BIAS: isize = 112; // magic number
    let mut seen_grid = [[false; 256]; 256]; // 64KiB "only"
    let mut head = Coord(0, 0);
    let mut tail = Coord(0, 0);
    seen_grid[NORMALIZE_BIAS as usize][NORMALIZE_BIAS as usize] = true; // initial tail position
    parse_input().for_each(|(c, n)| {
        // move the head
        match c {
            b'R' => *head.x_mut() += n,
            b'L' => *head.x_mut() -= n,
            b'U' => *head.y_mut() += n,
            b'D' => *head.y_mut() -= n,
            _ => unreachable!(),
        };
        for _ in 0..n {
            // check if we need to move the tail
            if head.is_adjacent_to(&tail) {
                // we don't need to do anything
                break;
            }

            match c {
                b'R' => {
                    *tail.x_mut() += 1;
                    *tail.y_mut() = *head.y();
                }
                b'L' => {
                    *tail.x_mut() -= 1;
                    *tail.y_mut() = *head.y();
                }
                b'U' => {
                    *tail.y_mut() += 1;
                    *tail.x_mut() = *head.x();
                }
                b'D' => {
                    *tail.y_mut() -= 1;
                    *tail.x_mut() = *head.x();
                }
                _ => unreachable!(),
            };

            // if we moved the tail, update the seen grid
            seen_grid[(tail.x() + NORMALIZE_BIAS) as usize][(tail.y() + NORMALIZE_BIAS) as usize] =
                true;
        }
    });
    seen_grid
        .iter()
        .flat_map(|l| l.iter())
        .filter(|&&b| b)
        .count()
}

pub fn part2() -> usize {
    const NORMALIZE_BIAS: isize = 112; // magic number
    let mut seen_grid = [[false; 256]; 256]; // 64KiB "only"
    let mut knots = [
        Coord(0, 0),
        Coord(0, 0),
        Coord(0, 0),
        Coord(0, 0),
        Coord(0, 0),
        Coord(0, 0),
        Coord(0, 0),
        Coord(0, 0),
        Coord(0, 0),
        Coord(0, 0),
    ];
    seen_grid[NORMALIZE_BIAS as usize][NORMALIZE_BIAS as usize] = true; // initial tail position
    parse_input().for_each(|(c, n)| {
        for _ in 0..n {
            // For some reason, moving the head movement out of the loop
            // makes the whole thing significantly slower.
            match c {
                b'R' => *knots[0].x_mut() += 1,
                b'L' => *knots[0].x_mut() -= 1,
                b'U' => *knots[0].y_mut() += 1,
                b'D' => *knots[0].y_mut() -= 1,
                _ => unreachable!(),
            };

            let knots_count = knots.len();
            for k in 1..knots_count {
                let last = knots[k - 1].clone(); // TODO: get rid of this clone
                let next = &mut knots[k];

                let (dx, dy) = (last.x() - next.x(), last.y() - next.y());

                // if head is two steps ahead but in the same row or col...
                if next.same_row_or_col(&last) && (dx == 2 || dx == -2 || dy == 2 || dy == -2) {
                    // ... the tail must also move one step in that direction
                    if dx > 0 {
                        *next.x_mut() += 1;
                    } else if dx < 0 {
                        *next.x_mut() -= 1;
                    } else if dy > 0 {
                        *next.y_mut() += 1;
                    } else if dy < 0 {
                        *next.y_mut() -= 1;
                    }
                } else if !next.is_adjacent_to(&last) {
                    if dx > 0 {
                        *next.x_mut() += 1;
                    }
                    if dx < 0 {
                        *next.x_mut() -= 1;
                    }
                    if dy > 0 {
                        *next.y_mut() += 1;
                    }
                    if dy < 0 {
                        *next.y_mut() -= 1;
                    }
                }

                if k == knots_count - 1 {
                    // we only care about the very tail
                    seen_grid[(next.x() + NORMALIZE_BIAS) as usize]
                        [(next.y() + NORMALIZE_BIAS) as usize] = true;
                }
            }
        }
    });
    seen_grid
        .iter()
        .flat_map(|l| l.iter())
        .filter(|&&b| b)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate test;

    use test::Bencher;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 5883);
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(part1);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 2367);
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(part2);
    }
}
