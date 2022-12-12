use pathfinding::matrix::Matrix;
use pathfinding::prelude::bfs;

const INPUT: &str = include_str!("../inputs/day12.txt");

fn parse_input() -> (Matrix<u8>, (usize, usize), (usize, usize)) {
    let mut map = Matrix::from_rows(INPUT.lines().map(str::bytes)).unwrap();
    let start = map.indices().find(|&coord| map[coord] == b'S').unwrap();
    let end = map.indices().find(|&coord| map[coord] == b'E').unwrap();
    map[start] = b'a';
    map[end] = b'z';
    (map, start, end)
}

pub fn part1() -> usize {
    let (ref map, start, end) = parse_input();
    bfs(
        &start,
        |&coord| {
            map.neighbours(coord, false)
                .filter(move |&p| map[p] <= map[coord] + 1)
        },
        |coord| *coord == end,
    )
    .map(|path| path.len() - 1)
    .unwrap()
}

pub fn part2() -> usize {
    let (ref map, _, end) = parse_input();
    // start at the end and find the shortest way to an 'a'
    bfs(
        &end,
        |&coord| {
            map.neighbours(coord, false)
                .filter(move |&p| map[coord] <= map[p] + 1) // reverse of part 1
        },
        |&coord| map[coord] == b'a',
    )
    .map(|path| path.len() - 1)
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate test;

    use test::Bencher;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 394);
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(part1);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 388);
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(part2);
    }
}
