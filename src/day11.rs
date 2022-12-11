use std::collections::VecDeque;

const INPUT: &str = include_str!("../inputs/day11.txt");

#[derive(Debug)]
enum Operation {
    /// new = old + n
    Add(usize),
    /// new = old * n
    Mul(usize),
    /// new = old * old
    Exp,
}

struct Monkey {
    starting_items: VecDeque<usize>,
    operation: Operation,
    test_divisible_by: usize,
    test_true: usize,
    test_false: usize,
}

impl Monkey {
    fn from_str_array(s: [&str; 7]) -> Self {
        let starting_items = s[1][18..]
            .as_bytes()
            .array_chunks::<2>()
            .step_by(2)
            .map(|b| unsafe {
                // SAFETY: the bytes came from a &str, so they are valid UTF-8
                std::str::from_utf8_unchecked(b)
            })
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<VecDeque<usize>>();
        let s2_bytes = s[2].as_bytes();
        let operation = match s2_bytes[23] {
            b'+' => Operation::Add(s[2][25..].parse::<usize>().unwrap()),
            b'*' => {
                if s2_bytes[25] == b'o' {
                    Operation::Exp
                } else {
                    Operation::Mul(s[2][25..].parse::<usize>().unwrap())
                }
            }
            _ => unreachable!(),
        };
        let test_divisible_by = s[3][21..].parse::<usize>().unwrap();
        let test_true = s[4][29..].parse::<usize>().unwrap();
        let test_false = s[5][30..].parse::<usize>().unwrap();
        Self {
            starting_items,
            operation,
            test_divisible_by,
            test_true,
            test_false,
        }
    }
}

fn parse_monkeys() -> Vec<Monkey> {
    INPUT
        .lines()
        .chain(std::iter::once("")) // array chunks doesn't like the single newline at the end, so we add one
        .array_chunks::<7>()
        .map(Monkey::from_str_array)
        .collect::<Vec<Monkey>>()
}

pub fn part1() -> usize {
    solve(parse_monkeys(), 20, |v| v / 3)
}

pub fn part2() -> usize {
    let monkeys = parse_monkeys();
    let modulo: usize = monkeys.iter().map(|m| m.test_divisible_by).product();
    solve(monkeys, 10_000, |v| v % modulo)
}

fn solve<F>(mut monkeys: Vec<Monkey>, rounds: usize, worry_level_modifier: F) -> usize
where
    F: Fn(usize) -> usize,
{
    let mut monkey_business = vec![0_usize; monkeys.len()]; // TODO: make this an array

    for _round in 0..rounds {
        for monkey_num in 0..monkeys.len() {
            for _ in 0..monkeys[monkey_num].starting_items.len() {
                let monkey = &mut monkeys[monkey_num];
                let mut item = monkey.starting_items.pop_front().unwrap();

                // inspect the item
                monkey_business[monkey_num] += 1;

                // increase worry level
                item = match monkey.operation {
                    Operation::Add(n) => item + n,
                    Operation::Mul(n) => item * n,
                    Operation::Exp => item * item,
                };

                // modify worry level
                item = worry_level_modifier(item);

                // throw the item to the next monkey
                let next_monkey = if item % monkeys[monkey_num].test_divisible_by == 0 {
                    monkeys[monkey_num].test_true
                } else {
                    monkeys[monkey_num].test_false
                };
                monkeys[next_monkey].starting_items.push_back(item);
            }
        }
    }

    monkey_business.sort_unstable();
    monkey_business[monkey_business.len() - 1] * monkey_business[monkey_business.len() - 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate test;

    use test::Bencher;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 316888);
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(part1);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 35270398814);
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(part2);
    }
}
