use std::simd::Simd;
use std::simd::SimdPartialOrd;
use std::simd::ToBitMask;

pub fn t() {}

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
    return sorted.len() + 1;
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

    return left + 1;
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

    return sorted_len + 1;
}

#[cfg(test)]
mod tests {
    use crate::simd_test::find_binary;
    use crate::simd_test::find_iter;
    use crate::simd_test::find_linear;
    use crate::simd_test::find_simd;

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
}
