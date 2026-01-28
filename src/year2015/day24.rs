//! # It Hangs in the Balance
//!
//! To simplify things assumes that the remaining items after the first best combination is found
//! can be split evenly.
//!
//! Sorts the weights in ascending order, then tries combinations of increasing size until a
//! match in found. This will be the answer since the package count is the smallest and the
//! quantum entanglement will also be the lowest.
use crate::util::parse::*;

pub fn parse(input: &str) -> Vec<usize> {
    input.iter_unsigned().collect()
}

pub fn part1(input: &[usize]) -> usize {
    arrangements(input, 3)
}

pub fn part2(input: &[usize]) -> usize {
    arrangements(input, 4)
}

fn arrangements(input: &[usize], groups: usize) -> usize {
    let goal = input.iter().sum::<usize>() / groups;

    let mut minimum = vec![u32::MAX; goal + 1];
    minimum[0] = 0;

    let mut qe = vec![usize::MAX; goal + 1];
    qe[0] = 1;

    for &item in input {
        for i in (item..goal + 1).rev() {
            let take = minimum[i - item].saturating_add(1);
            let not_take = minimum[i];

            if take < not_take {
                qe[i] = item.saturating_mul(qe[i - item]);
                minimum[i] = take;
            } else if take == not_take {
                qe[i] = qe[i].min(item.saturating_mul(qe[i - item]));
            }
        }
    }

    qe[goal]    
}
