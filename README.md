# AoC 2022 in Rust

This is my attempt at the Advent of Code 2022 in Rust.
The goal is, to get fast execution times.

## Performance
```plain
test day1::tests::bench_part1      ... bench:      25,180 ns/iter (+/- 303)
test day1::tests::bench_part2      ... bench:      25,235 ns/iter (+/- 512)
test day2::tests::bench_part1      ... bench:      27,804 ns/iter (+/- 342)
test day2::tests::bench_part2      ... bench:      25,111 ns/iter (+/- 689)
test day3::tests::bench_part1      ... bench:       9,387 ns/iter (+/- 273)
test day3::tests::bench_part2      ... bench:      17,722 ns/iter (+/- 771)
test day3::tests::bench_part2_simd ... bench:      17,002 ns/iter (+/- 386)
test day4::tests::bench_part1      ... bench:      38,741 ns/iter (+/- 1,645)
test day4::tests::bench_part2      ... bench:      37,207 ns/iter (+/- 605)
test day5::tests::bench_part1      ... bench:      24,852 ns/iter (+/- 521)
test day5::tests::bench_part2      ... bench:      38,104 ns/iter (+/- 953)
test day6::tests::bench_part1      ... bench:       1,403 ns/iter (+/- 35)
test day6::tests::bench_part2      ... bench:       7,052 ns/iter (+/- 193)
test day7::tests::bench_part1      ... bench:     120,039 ns/iter (+/- 5,084)
test day7::tests::bench_part2      ... bench:     120,765 ns/iter (+/- 3,306)
test day8::tests::bench_part1      ... bench:      64,805 ns/iter (+/- 1,115)
test day8::tests::bench_part2      ... bench:     179,102 ns/iter (+/- 3,265)
```

Benchmarks are included and done via `cargo bench`.
The results posted here were taken on a 2022 MacBook Air M2 with power attached.

If you want to make something faster, feel free to send a PR my way.