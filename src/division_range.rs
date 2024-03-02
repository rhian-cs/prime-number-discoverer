use itertools::Itertools;
use std::ops::Range;

const DIVISION_MIN_DIVISOR: u32 = 2;

pub fn partitioned_division_ranges(n: u32, partition_count: u32) -> Vec<Range<u32>> {
    let max_divisor = division_max_divisor(n);
    let range = DIVISION_MIN_DIVISOR..max_divisor;

    if partition_count == 1 || partition_count > (max_divisor - 2) {
        return [range].into();
    }

    let partition_size = partition_size(max_divisor, partition_count);

    range
        .step_by(partition_size)
        .tuple_windows()
        .map(|(a, b)| a..b)
        .collect()
}

fn partition_size(max: u32, partition_count: u32) -> usize {
    if partition_count > max {
        return max as usize;
    }

    ((max - 2) / partition_count) as usize
}

fn division_max_divisor(n: u32) -> u32 {
    (n / 2) + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_vec(ranges: Vec<Range<u32>>) -> Vec<Vec<u32>> {
        ranges
            .into_iter()
            .map(|range| range.collect())
            .collect_vec()
    }

    mod partition_size_fn {
        use super::*;

        #[test]
        fn it_partitions_correctly() {
            assert_eq!(partition_size(50, 2), 24);
            assert_eq!(partition_size(100, 2), 49);

            assert_eq!(partition_size(50, 4), 12);
            assert_eq!(partition_size(100, 4), 24);
        }
    }

    mod partitioned_division_ranges_fn {
        use super::*;

        #[test]
        fn it_returns_full_range_for_1_partition() {
            assert_eq!(to_vec(partitioned_division_ranges(1, 1)), [[]]);
            assert_eq!(to_vec(partitioned_division_ranges(2, 1)), [[]]);
            assert_eq!(to_vec(partitioned_division_ranges(3, 1)), [[]]);
            assert_eq!(to_vec(partitioned_division_ranges(4, 1)), [[2]]);
            assert_eq!(to_vec(partitioned_division_ranges(5, 1)), [[2]]);
            assert_eq!(to_vec(partitioned_division_ranges(6, 1)), [[2, 3]]);
            assert_eq!(to_vec(partitioned_division_ranges(7, 1)), [[2, 3]]);
            assert_eq!(to_vec(partitioned_division_ranges(8, 1)), [[2, 3, 4]]);
            assert_eq!(to_vec(partitioned_division_ranges(9, 1)), [[2, 3, 4]]);
            assert_eq!(to_vec(partitioned_division_ranges(10, 1)), [[2, 3, 4, 5]]);
        }

        #[test]
        fn it_returns_two_range_partitions_for_8() {
            let result = partitioned_division_ranges(8, 2);

            let expected_result = [2..3, 3..4];

            assert_eq!(result, expected_result);
        }

        #[test]
        fn it_returns_two_range_partitions_for_100() {
            let result = partitioned_division_ranges(100, 2);

            let expected_result = [2..26, 26..50];

            assert_eq!(result, expected_result);
        }

        #[test]
        fn it_returns_four_range_partitions_for_100() {
            let result = partitioned_division_ranges(100, 4);

            let expected_result = [2..14, 14..26, 26..38, 38..50];

            assert_eq!(result, expected_result);
        }

        #[test]
        fn it_returns_single_partition_for_partition_count_larger_than_or_equal_to_max_divisor() {
            assert_eq!(partitioned_division_ranges(5, 2), [2..3]);
            assert_eq!(partitioned_division_ranges(5, 3), [2..3]);
            assert_eq!(partitioned_division_ranges(5, 4), [2..3]);
            assert_eq!(partitioned_division_ranges(5, 5), [2..3]);
            assert_eq!(partitioned_division_ranges(5, 10), [2..3]);

            assert_eq!(partitioned_division_ranges(8, 4), [2..5]);
            assert_eq!(partitioned_division_ranges(8, 10), [2..5]);

            assert_eq!(partitioned_division_ranges(10, 100), [2..6]);
            assert_eq!(partitioned_division_ranges(10, 1000), [2..6]);
            assert_eq!(partitioned_division_ranges(10, 10000), [2..6]);
        }
    }

    mod division_max_divisor_fn {
        use super::*;

        #[test]
        fn it_calculates_half_of_number() {
            assert_eq!(division_max_divisor(3), 2);
            assert_eq!(division_max_divisor(4), 3);
            assert_eq!(division_max_divisor(5), 3);
            assert_eq!(division_max_divisor(6), 4);
            assert_eq!(division_max_divisor(7), 4);
            assert_eq!(division_max_divisor(8), 5);
            assert_eq!(division_max_divisor(9), 5);
            assert_eq!(division_max_divisor(10), 6);
        }
    }
}
