use std::collections::VecDeque;

const INPUT: &str = include_str!("../inputs/day5.txt");

#[derive(Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl Move {
    fn from_line(line: &str) -> Self {
        let mut split = line.split(|c| c == ' ');
        let count = split.nth(1).unwrap().parse::<usize>().unwrap();
        let from = split.nth(1).unwrap().parse::<usize>().unwrap();
        let to = split.nth(1).unwrap().parse::<usize>().unwrap();
        Self { count, from, to }
    }
}

pub fn part1() -> String {
    let mut stacks: Vec<VecDeque<char>> = vec![VecDeque::new(); 9];
    INPUT.lines().take(8).for_each(|l| {
        let mut elements = [None; 9];
        for (i, p) in (1..l.len()).step_by(4).enumerate() {
            let c = l.chars().nth(p).unwrap();
            elements[i] = if c == ' ' { None } else { Some(c) };
        }
        for (i, e) in elements
            .into_iter()
            .enumerate()
            .filter(|(_, e)| e.is_some())
        {
            stacks[i].push_back(e.unwrap());
        }
    });

    INPUT.lines().skip(10).map(Move::from_line).for_each(|m| {
        for _ in 0..m.count {
            let elem = stacks[m.from - 1].pop_front().unwrap();
            stacks[m.to - 1].push_front(elem);
        }
    });

    let mut res = String::with_capacity(stacks.len());
    stacks.iter().for_each(|s| res.push(*s.front().unwrap()));
    res
}

pub fn part2() -> String {
    let mut stacks: Vec<VecDeque<char>> = vec![VecDeque::new(); 9];
    INPUT.lines().take(8).for_each(|l| {
        let mut elements = [None; 9];
        for (i, p) in (1..l.len()).step_by(4).enumerate() {
            let c = l.chars().nth(p).unwrap();
            elements[i] = if c == ' ' { None } else { Some(c) };
        }
        for (i, e) in elements
            .into_iter()
            .enumerate()
            .filter(|(_, e)| e.is_some())
        {
            stacks[i].push_back(e.unwrap());
        }
    });

    INPUT.lines().skip(10).map(Move::from_line).for_each(|m| {
        let mut elems: VecDeque<char> = VecDeque::with_capacity(m.count);
        for _ in 0..m.count {
            let elem = stacks[m.from - 1].pop_front().unwrap();
            elems.push_front(elem);
        }
        elems
            .into_iter()
            .for_each(|c| stacks[m.to - 1].push_front(c));
    });

    let mut res = String::with_capacity(stacks.len());
    stacks.iter().for_each(|s| res.push(*s.front().unwrap()));
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate test;
    use test::Bencher;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), "SBPQRSCDF");
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(part1);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), "RGLVRCQSB");
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(part2);
    }
}
