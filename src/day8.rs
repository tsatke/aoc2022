use std::mem::swap;
use std::ops::Deref;

const INPUT: &str = include_str!("../inputs/day8.txt");

const LINES: usize = 99;
const COLS: usize = 99;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Coordinate(u8, u8);

impl Coordinate {
    fn new(line: u8, col: u8) -> Self {
        Self(line, col)
    }

    fn transpose(&mut self) {
        swap(&mut self.0, &mut self.1);
    }
}

#[derive(Debug)]
struct Grid([[u8; COLS]; LINES]);

impl From<[[u8; COLS]; LINES]> for Grid {
    fn from(v: [[u8; COLS]; LINES]) -> Self {
        Self(v)
    }
}

impl Grid {
    fn transpose(&self) -> Self {
        let mut rotated = Self::from([[0_u8; COLS]; LINES]);

        for (row, line) in self.0.iter().enumerate() {
            for (col, c) in line.iter().enumerate() {
                rotated.0[col][row] = *c;
            }
        }

        rotated
    }

    fn lines(&self) -> impl Iterator<Item = Line> {
        self.0.iter().map(|l| Line(l))
    }
}

#[derive(Debug)]
struct Line<'a>(&'a [u8; COLS]);

impl<'a> Deref for Line<'a> {
    type Target = &'a [u8; COLS];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn get_grid() -> Grid {
    let mut grid = [[0_u8; COLS]; LINES];
    INPUT.lines().enumerate().for_each(|(line_num, l)| {
        l.bytes()
            .map(|b| b - b'0')
            .enumerate()
            .for_each(|(col_num, c)| {
                grid[line_num][col_num] = c;
            })
    });
    Grid::from(grid)
}

fn get_visible(grid: &Grid) -> impl Iterator<Item = Coordinate> + '_ {
    grid.lines()
        .enumerate()
        .take(LINES - 1)
        .skip(1)
        .flat_map(|(line_num, line)| {
            let mut highest_left = line[0];
            let mut highest_right = line[line.len() - 1];
            let visible = Iterator::chain(
                line.iter().enumerate().filter_map(|(col_num, &c)| {
                    if c > highest_left {
                        highest_left = c;
                        Some(Coordinate::new(line_num as u8, col_num as u8))
                    } else {
                        None
                    }
                }),
                line.iter().enumerate().rev().filter_map(|(col_num, &c)| {
                    if c > highest_right {
                        highest_right = c;
                        Some(Coordinate::new(line_num as u8, col_num as u8))
                    } else {
                        None
                    }
                }),
            )
            .collect::<Vec<Coordinate>>(); // TODO: remove this collect
            visible.into_iter()
        })
}

pub fn part1() -> usize {
    const VISIBLE_FROM_EDGE: usize = (COLS + LINES) * 2 - 4;
    let grid = get_grid();
    let mut visible = get_visible(&grid)
        .chain(get_visible(&grid.transpose()).map(|mut c| {
            c.transpose();
            c
        }))
        .collect::<Vec<Coordinate>>();
    // all visible coordinates refer to `grid` (regarding transpositions)
    visible.sort_unstable();
    visible.dedup();

    visible.iter().count() + VISIBLE_FROM_EDGE
}

pub fn part2() -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate test;

    use test::Bencher;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 1827);
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(part1);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 0);
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(part2);
    }
}
