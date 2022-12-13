use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};

const INPUT: &str = include_str!("../inputs/day13.txt");

#[derive(Debug, PartialEq, Eq)]
enum Value {
    List(Vec<Value>),
    Number(u8),
}

impl Value {
    fn from_str(s: &str) -> Self {
        if &s[0..1] == "[" {
            let mut stack: i32 = 0;
            // pre-allocation helps here
            let mut vec = Vec::with_capacity(5);
            s[1..s.len() - 1]
                .split(|c| {
                    if c == '[' {
                        stack += 1
                    } else if c == ']' {
                        stack -= 1
                    }
                    c == ',' && stack == 0
                })
                .filter_map(|s| (!s.is_empty()).then(|| Self::from_str(s)))
                .for_each(|e| vec.push(e));
            Self::List(vec)
        } else {
            Self::Number(s.parse().unwrap())
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Value::*;

        fn first_number(v: &Value) -> Option<u8> {
            match v {
                Number(n) => Some(*n),
                List(l) => {
                    if l.is_empty() {
                        None
                    } else {
                        first_number(&l[0])
                    }
                }
            }
        }
        match (self, other) {
            (Number(a), Number(b)) => a.partial_cmp(b),
            (List(a), List(b)) => a.partial_cmp(b),
            (l @ List(_), r @ Number(_)) => Self::partial_cmp(r, l).map(Ordering::reverse),
            (Number(n), l @ List(inner)) => {
                if let Some(other) = first_number(l) {
                    Some(match n.partial_cmp(&other) {
                        Some(Equal) if inner.len() > 1 => Less,
                        Some(other) => other,
                        None => return None,
                    })
                } else {
                    Some(Greater)
                }
            }
        }
    }
}

pub fn part1() -> usize {
    INPUT
        .lines()
        .filter(|s| !s.is_empty())
        .map(Value::from_str)
        .array_chunks::<2>()
        .enumerate()
        .filter_map(|(n, p)| (p[0] <= p[1]).then_some(n + 1))
        .sum()
}

pub fn part2() -> usize {
    use Value::*;

    let div1 = List(vec![List(vec![Number(2)])]);
    let div2 = List(vec![List(vec![Number(6)])]);
    let res = INPUT
        .lines()
        .filter(|l| !l.is_empty())
        .map(Value::from_str)
        .fold((1, 2), |mut acc, v| {
            if v <= div1 {
                acc.0 += 1;
            }
            if v <= div2 {
                acc.1 += 1;
            }
            acc
        });
    res.0 * res.1
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate test;

    use test::Bencher;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 5588);
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(part1);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 23958);
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(part2);
    }
}
