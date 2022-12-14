use itertools::Itertools;
use std::ops::{Add, Index, IndexMut};

const INPUT: &str = include_str!("../inputs/day14.txt");

const SAND_SPAWN: Coord = Coord::new(500 - X_BIAS, 0);
const COLS: usize = 341;
const ROWS: usize = 171;
const X_BIAS: usize = 330;

/// A normalized coordinate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord(usize, usize);

impl Coord {
    const fn new(x: usize, y: usize) -> Self {
        Self(y, x)
    }

    const fn row(&self) -> usize {
        self.0
    }

    const fn col(&self) -> usize {
        self.1
    }

    fn row_mut(&mut self) -> &mut usize {
        &mut self.0
    }

    fn col_mut(&mut self) -> &mut usize {
        &mut self.1
    }
}

enum Direction {
    Down,
    Left,
    Right,
}

impl Add<Direction> for Coord {
    type Output = Coord;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::Down => Coord::new(self.col(), self.row() + 1),
            Direction::Left => Coord::new(self.col() - 1, self.row()),
            Direction::Right => Coord::new(self.col() + 1, self.row()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Location {
    Spawn, // Spawn is blocked
    Map,
    Floor,
}

struct Interpolate {
    start: Coord,
    end: Coord,
    done: bool,
}

impl Interpolate {
    pub const fn between(start: Coord, end: Coord) -> Self {
        Self {
            start,
            end,
            done: false,
        }
    }
}

impl Iterator for Interpolate {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        if self.start == self.end {
            self.done = true;
            return Some(self.start);
        }

        let old = self.start;
        if self.start.row() < self.end.row() {
            *self.start.row_mut() += 1;
        } else if self.start.row() > self.end.row() {
            *self.start.row_mut() -= 1;
        } else if self.start.col() < self.end.col() {
            *self.start.col_mut() += 1;
        } else if self.start.col() > self.end.col() {
            *self.start.col_mut() -= 1;
        }
        Some(old)
    }
}

fn drop_sand(map: &mut Grid, floor: usize) -> Location {
    let mut sand = SAND_SPAWN;
    if map[sand] {
        return Location::Spawn;
    }

    loop {
        let below = sand + Direction::Down;
        if below.row() >= floor {
            map[sand] = true;
            return Location::Floor;
        }

        if !map[below] {
            // below is free, go there
            sand = below;
            continue;
        }
        let below_left = below + Direction::Left;
        if !map[below_left] {
            // below left is free, go there
            sand = below_left;
            continue;
        }
        let below_right = below + Direction::Right;
        if !map[below_right] {
            // below right is free, go there
            sand = below_right;
            continue;
        }

        // all spots are blocked
        map[sand] = true;
        return Location::Map;
    }
}

struct Grid([[bool; COLS]; ROWS]);

impl Index<Coord> for Grid {
    type Output = bool;

    fn index(&self, index: Coord) -> &Self::Output {
        &self.0[index.row()][index.col()]
    }
}

impl IndexMut<Coord> for Grid {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        &mut self.0[index.row()][index.col()]
    }
}

fn build_sparse_map() -> (Grid, usize) {
    let mut grid: Grid = Grid([[false; COLS]; ROWS]);
    let mut max_y = 0_usize;

    INPUT.lines().for_each(|l| {
        l.split(" -> ")
            .flat_map(|s| s.split(|c| c == ','))
            .array_chunks::<2>()
            .map(|[x, y]| {
                let x = x.parse::<usize>().unwrap();
                let y = y.parse::<usize>().unwrap();
                if y > max_y {
                    max_y = y;
                }
                Coord::new(x - X_BIAS, y)
            })
            .tuple_windows::<(Coord, Coord)>()
            .flat_map(|(l, r)| Interpolate::between(l, r))
            .for_each(|c| {
                grid[c] = true;
            });
    });
    (grid, max_y)
}

pub fn part1() -> usize {
    let (mut map, max_y) = build_sparse_map();

    let mut count: usize = 0;
    while matches!(drop_sand(&mut map, max_y + 1), Location::Map) {
        count += 1;
    }

    count
}

pub fn part2() -> usize {
    let (mut map, max_y) = build_sparse_map();
    let floor_y = max_y + 2;

    let mut count: usize = 0;
    while !matches!(drop_sand(&mut map, floor_y), Location::Spawn) {
        count += 1;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate test;

    use test::Bencher;

    #[test]
    fn test_interpolate() {
        assert_eq!(
            Interpolate::between(Coord::new(0, 0), Coord::new(0, 0)).collect::<Vec<_>>(),
            vec![Coord::new(0, 0)]
        );
        assert_eq!(
            Interpolate::between(Coord::new(0, 0), Coord::new(0, 1)).collect::<Vec<_>>(),
            vec![Coord::new(0, 0), Coord::new(0, 1)]
        );
        assert_eq!(
            Interpolate::between(Coord::new(0, 0), Coord::new(0, 5)).collect::<Vec<_>>(),
            vec![
                Coord::new(0, 0),
                Coord::new(0, 1),
                Coord::new(0, 2),
                Coord::new(0, 3),
                Coord::new(0, 4),
                Coord::new(0, 5)
            ]
        );
        assert_eq!(
            Interpolate::between(Coord::new(0, 5), Coord::new(0, 0)).collect::<Vec<_>>(),
            vec![
                Coord::new(0, 5),
                Coord::new(0, 4),
                Coord::new(0, 3),
                Coord::new(0, 2),
                Coord::new(0, 1),
                Coord::new(0, 0),
            ]
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 843);
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(part1);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 27625);
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(part2);
    }
}
