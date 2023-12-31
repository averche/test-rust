#[cfg(test)]
mod tests {
    use std::simd::Simd;
    use std::simd::SimdPartialOrd;
    use std::simd::ToBitMask;

    fn find_iter(sorted: &[i32], target: i32) -> usize {
        return sorted
            .iter()
            .position(|&el| target < el)
            .unwrap_or(sorted.len())
            + 1;
    }

    fn find_linear(sorted: &[i32], target: i32) -> usize {
        for (i, &el) in sorted.iter().enumerate() {
            if target < el {
                return i + 1;
            }
        }

        sorted.len() + 1
    }

    fn find_binary(sorted: &[i32], target: i32) -> usize {
        let mut left = 0;
        let mut right = sorted.len();

        while left < right {
            let mid = left + (right - left) / 2;

            if sorted[mid] <= target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left + 1
    }

    fn find_simd(sorted: &[i32], target: i32) -> usize {
        let sorted_len = sorted.len();

        let target_8: Simd<i32, 8> = Simd::splat(target);

        let mut i: usize = 0;

        while i < sorted_len {
            if sorted_len - i < 8 {
                // this is the last batch: iterate
                while i < sorted_len {
                    if target < sorted[i] {
                        return i + 1;
                    }
                    i += 1;
                }
            } else {
                // simd
                let next_8: Simd<i32, 8> = Simd::from_slice(&sorted[i..(i + 8)]);

                let matches = target_8.simd_lt(next_8);
                if matches.any() {
                    return i + (matches.to_bitmask().trailing_zeros() as usize) + 1;
                }

                i += 8;
            }
        }

        sorted_len + 1
    }

    use test::black_box;
    use test::Bencher;

    #[test]
    fn test_find() {
        let vec1 = [8, 14, 23, 34, 42, 49, 53, 59, 63, 72];

        for &(expected, target) in [
            (1, 0),
            (1, 5),
            (4, 25),
            (5, 34),
            (6, 45),
            (7, 49),
            (11, 100), //
        ]
        .iter()
        {
            assert_eq!(expected, find_binary(&vec1, target));
            assert_eq!(expected, find_iter(&vec1, target));
            assert_eq!(expected, find_linear(&vec1, target));
            assert_eq!(expected, find_simd(&vec1, target));
        }
    }

    #[bench]
    fn bench_80_find_binary(b: &mut Bencher) {
        let vec1: Vec<i32> = (0..80).map(|x| x * 100 + 20).collect();
        let vec2: Vec<i32> = (0..80).map(|x| x * 100 + 50).collect();

        b.iter(|| {
            for _ in 0..10 {
                for target in vec1.iter() {
                    black_box(find_binary(&vec2, *target));
                }
                for target in vec1.iter().rev() {
                    black_box(find_binary(&vec2, *target));
                }
                for target in vec2.iter() {
                    black_box(find_binary(&vec2, *target));
                }
                for target in vec2.iter().rev() {
                    black_box(find_binary(&vec2, *target));
                }
            }
        });
    }

    #[bench]
    fn bench_80_find_iter(b: &mut Bencher) {
        let vec1: Vec<i32> = (0..80).map(|x| x * 100 + 20).collect();
        let vec2: Vec<i32> = (0..80).map(|x| x * 100 + 50).collect();

        b.iter(|| {
            for _ in 0..10 {
                for target in vec1.iter() {
                    black_box(find_iter(&vec2, *target));
                }
                for target in vec1.iter().rev() {
                    black_box(find_iter(&vec2, *target));
                }
                for target in vec2.iter() {
                    black_box(find_iter(&vec2, *target));
                }
                for target in vec2.iter().rev() {
                    black_box(find_iter(&vec2, *target));
                }
            }
        });
    }

    #[bench]
    fn bench_80_find_linear(b: &mut Bencher) {
        let vec1: Vec<i32> = (0..80).map(|x| x * 100 + 20).collect();
        let vec2: Vec<i32> = (0..80).map(|x| x * 100 + 50).collect();

        b.iter(|| {
            for _ in 0..10 {
                for target in vec1.iter() {
                    black_box(find_linear(&vec2, *target));
                }
                for target in vec1.iter().rev() {
                    black_box(find_linear(&vec2, *target));
                }
                for target in vec2.iter() {
                    black_box(find_linear(&vec2, *target));
                }
                for target in vec2.iter().rev() {
                    black_box(find_linear(&vec2, *target));
                }
            }
        });
    }

    #[bench]
    fn bench_80_find_simd(b: &mut Bencher) {
        let vec1: Vec<i32> = (0..80).map(|x| x * 100 + 20).collect();
        let vec2: Vec<i32> = (0..80).map(|x| x * 100 + 50).collect();

        b.iter(|| {
            for _ in 0..10 {
                for target in vec1.iter() {
                    black_box(find_simd(&vec2, *target));
                }
                for target in vec1.iter().rev() {
                    black_box(find_simd(&vec2, *target));
                }
                for target in vec2.iter() {
                    black_box(find_simd(&vec2, *target));
                }
                for target in vec2.iter().rev() {
                    black_box(find_simd(&vec2, *target));
                }
            }
        });
    }
}
