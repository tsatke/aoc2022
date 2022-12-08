use std::collections::HashMap;

const INPUT: &str = include_str!("../inputs/day7.txt");

// TODO: the tree iteration is a depth first search, so this may be a thing to optimize for

fn compute_sizes() -> HashMap<Vec<&'static str>, usize> {
    let mut sizes: HashMap<Vec<&str>, usize> = HashMap::new();
    let mut cwd: Vec<&str> = Vec::new();
    INPUT.lines().for_each(|l| {
        let bytes = l.as_bytes();
        match bytes[0] {
            b'$' => {
                if bytes[2] == b'c' {
                    match &bytes[5] {
                        b'.' => {
                            cwd.pop().unwrap();
                        }
                        _ => {
                            cwd.push(&l[5..]);
                            // TODO: the hash map could maybe hold a &Vec instead of a Vec, which would allow us, to store references instead of cloning
                            sizes.insert(cwd.clone(), 0); // TODO: get rid of the clone
                        }
                    }
                }
            }
            b'd' => {} // dir
            _ => {
                let size = l.split(' ').next().unwrap().parse::<usize>().unwrap();

                for i in (1..cwd.len() + 1).rev() {
                    *sizes.get_mut(&cwd[0..i]).unwrap() += size;
                }
            }
        }
    });
    sizes
}

pub fn part1() -> usize {
    let sizes = compute_sizes();
    sizes.values().filter(|&&v| v <= 100000).sum::<usize>()
}

pub fn part2() -> usize {
    let sizes = compute_sizes();
    let root_size = sizes.get(&vec!["/"]).unwrap();
    let needed = 70000000 - 30000000;
    let threshold = root_size - needed;
    *sizes.values().filter(|&&v| v >= threshold).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate test;

    use test::Bencher;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 1583951);
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(part1);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 214171);
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(part2);
    }
}
