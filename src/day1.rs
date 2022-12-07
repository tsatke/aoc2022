const INPUT: &str = include_str!("../inputs/day1.txt");

#[derive(Copy, Clone, Eq, PartialEq)]
enum Value {
    Value(isize),
    Separator,
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        if v.is_empty() {
            Self::Separator
        } else {
            Self::Value(v.parse().unwrap())
        }
    }
}

struct CaloricChunks<I>
where
    I: Iterator<Item = Value>,
{
    inner: I,
}

impl<I> From<I> for CaloricChunks<I>
where
    I: Iterator<Item = Value>,
{
    fn from(i: I) -> Self {
        Self { inner: i }
    }
}

impl<I> Iterator for CaloricChunks<I>
where
    I: Iterator<Item = Value>,
{
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        let v = self
            .inner
            .by_ref()
            .map_while(|v| match v {
                Value::Value(num) => Some(num),
                _ => None,
            })
            .sum();
        if v == 0 {
            None
        } else {
            Some(v)
        }
    }
}

pub fn part1() -> isize {
    CaloricChunks::from(INPUT.lines().map(Value::from))
        .max()
        .unwrap()
}

pub fn part2() -> isize {
    let mut biggest = [isize::MIN; 3];
    CaloricChunks::from(INPUT.lines().map(Value::from)).for_each(|v| {
        let index = get_index_of_lowest(&biggest);
        if v > biggest[index] {
            biggest[index] = v;
        }
    });
    biggest.iter().sum()
}

fn get_index_of_lowest(values: &[isize]) -> usize {
    let mut lowest = isize::MAX;
    let mut index = 0;
    for (i, v) in values.iter().enumerate() {
        if v < &lowest {
            lowest = *v;
            index = i;
        }
    }
    index
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate test;
    use test::Bencher;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 68442);
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(part1);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 204837);
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(part2);
    }
}
