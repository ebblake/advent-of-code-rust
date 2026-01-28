//! # No Such Thing as Too Much
//!
//! Given `n` items the number of possible subsets is `2ⁿ`. We could brute force through each
//! subset by iterating from 0 to 2ⁿ using the binary bits to indicate if a container is present.
//! This will work but is a little slow as there are 20 containers, giving 2²⁰ = 1048576
//! combinations to check.
//!
//! Tackling this with dynamic programming provides a much faster approach.
use crate::util::parse::*;

pub fn parse(input: &str) -> Vec<usize> {
    input.iter_unsigned().collect()
}

pub fn part1(input: &[usize]) -> u32 {
    part1_testable(input, 150)
}

pub fn part2(input: &[usize]) -> u32 {
    part2_testable(input, 150)
}

pub fn part1_testable(input: &[usize], goal: usize) -> u32 {
    let mut ways = vec![0; goal + 1];
    ways[0] = 1;

    for &item in input {
        for i in (item..goal + 1).rev() {
            ways[i] += ways[i - item];
        }
    }

    ways[goal]
}

pub fn part2_testable(input: &[usize], goal: usize) -> u32 {
    let mut ways = vec![0; goal + 1];
    ways[0] = 1;

    let mut minimum = vec![u32::MAX; goal + 1];
    minimum[0] = 0;

    for &item in input {
        for i in (item..goal + 1).rev() {
            let take = minimum[i - item].saturating_add(1);
            let not_take = minimum[i];

            if take < not_take {
                ways[i] = ways[i - item];
                minimum[i] = take;
            } else if take == not_take {
                ways[i] += ways[i - item];
            }
        }
    }

    ways[goal]
}
