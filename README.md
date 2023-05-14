# test-rust

```sh
cargo bench

...
test simd_test::tests::bench_find_binary ... bench:      19,037 ns/iter (+/- 775)
test simd_test::tests::bench_find_iter   ... bench:      54,157 ns/iter (+/- 1,121)
test simd_test::tests::bench_find_linear ... bench:      56,291 ns/iter (+/- 2,847)
test simd_test::tests::bench_find_simd   ... bench:      18,820 ns/iter (+/- 474)
```
