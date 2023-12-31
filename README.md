# test-rust

## SIMD Benchmarks

```sh
$ cargo bench

...
test simd_test::tests::bench_100_find_binary ... bench:      28,390 ns/iter (+/- 1,938)
test simd_test::tests::bench_100_find_iter   ... bench:      89,213 ns/iter (+/- 7,612)
test simd_test::tests::bench_100_find_linear ... bench:      88,746 ns/iter (+/- 7,211)
test simd_test::tests::bench_100_find_simd   ... bench:      24,847 ns/iter (+/- 3,222)
```
