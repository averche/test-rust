# test-rust

## SIMD Benchmarks

```sh
$ cargo bench

...
test simd_test::tests::bench_80_find_binary ... bench:      19,356 ns/iter (+/- 1,299)
test simd_test::tests::bench_80_find_iter   ... bench:      54,597 ns/iter (+/- 3,170)
test simd_test::tests::bench_80_find_linear ... bench:      55,033 ns/iter (+/- 1,315)
test simd_test::tests::bench_80_find_simd   ... bench:      18,933 ns/iter (+/- 1,332)
```
