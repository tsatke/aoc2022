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

    fn transpose(&self) -> Coordinate {
        Self(self.1, self.0)
    }

    fn line(&self) -> usize {
        self.0 as usize
    }

    fn col(&self) -> usize {
        self.1 as usize
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

    fn coordinates(&self) -> impl Iterator<Item = Coordinate> + '_ {
        self.lines().enumerate().flat_map(|(line_num, line)| {
            line.iter()
                .enumerate()
                .map(move |(col_num, _)| Coordinate::new(line_num as u8, col_num as u8))
        })
    }

    fn height_at(&self, coord: Coordinate) -> u8 {
        self.line(coord.line()).0[coord.col()]
    }

    fn line(&self, line: usize) -> Line {
        Line(&self.0[line])
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
        .chain(get_visible(&grid.transpose()).map(|c| c.transpose()))
        .collect::<Vec<Coordinate>>();
    visible.sort_unstable();
    visible.dedup();

    visible.len() + VISIBLE_FROM_EDGE
}

fn scenic_score_row(g: &Grid, loc: Coordinate) -> usize {
    let height = g.height_at(loc);
    let line = g.line(loc.line());
    let mut score_right: usize = 0;
    for &c in line.iter().skip(loc.col() + 1) {
        score_right += 1;
        if c >= height {
            break;
        }
    }
    let mut score_left: usize = 0;
    for &c in line.iter().rev().skip(COLS - loc.col()) {
        score_left += 1;
        if c >= height {
            break;
        }
    }
    score_right * score_left
}

pub fn part2() -> usize {
    let grid = get_grid();
    let grid_transp = grid.transpose();
    let scenic_score = |loc: Coordinate| -> usize {
        let row = scenic_score_row(&grid, loc);
        let col = scenic_score_row(&grid_transp, loc.transpose());
        row * col
    };
    grid.coordinates()
        .map(|coord| scenic_score(coord))
        .max()
        .unwrap()
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
        assert_eq!(part2(), 335580);
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(part2);
    }
}
