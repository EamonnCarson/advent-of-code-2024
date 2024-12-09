use std::ops::Index;

use itertools::Itertools;

use crate::global_imports::*;

pub fn parse_line(string: &str) -> Vec<i32> {
    return string
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect()
}

enum Safety {
    Ascending(i32),
    Descending(i32),
}

pub fn is_safe(seq: &Vec<i32>) -> bool {
    if seq.len() <= 1 {
        panic!()
    } else {
        let ascending = 0 < seq[1] - seq[0];
        return seq.iter()
            .tuple_windows::<(_,_)>()
            .map(|(x, y)| y - x)
            .all(|diff| (0 < diff) == ascending && diff.abs() >= 1 && diff.abs() <= 3);
    }
}

pub fn is_safe_with_dampener(seq: &Vec<i32>) -> bool {
    for skip_index in 0..seq.len() {
        let diffs: Vec<i32> = (0..seq.len())
            .filter(|&index| index != skip_index)
            .map(|index| seq[index])
            .tuple_windows::<(_,_)>()
            .map(|(x, y)| y - x)
            .collect();
        let num_ascending = diffs
            .iter()
            .filter(|&&x| x > 0)
            .count();
        let asc_or_desc = num_ascending == 0 || num_ascending == diffs.len();
        let small_step = diffs.iter().all(|&x| x.abs() >= 1 && x.abs() <= 3);
        if asc_or_desc && small_step {
            return true;
        }
    }
    return false;
}

pub struct Day02;
impl Day for Day02 {
    fn star1(&self, input: String) -> String {
        let numbers: Vec<Vec<i32>> = input
            .lines()
            .map(parse_line)
            .collect();
        let safe_sequences: Vec<&Vec<i32>> = numbers.iter()
            .filter(|v| is_safe(v))
            .collect();
        let num_safe = safe_sequences.len();
        num_safe.to_string()
    }

    fn star2(&self, input: String) -> String {
        let numbers: Vec<Vec<i32>> = input
            .lines()
            .map(parse_line)
            .collect();
        log!("{:?}", numbers[0]);
        let safe_sequences: Vec<&Vec<i32>> = numbers.iter()
            .filter(|v| is_safe_with_dampener(v))
            .collect();
        let num_safe = safe_sequences.len();
        for i in 0..10 {
            log!("{:?}", safe_sequences[i]);
        }
        num_safe.to_string()
    }

    fn test_star2(&self) -> String {
        let numbers: Vec<Vec<i32>> = [
            [8, 11, 13, 14, 15, 18, 17].to_vec(),
            [17, 18, 15, 14, 13, 11, 8].to_vec(),
            [17, 18, 14, 14, 13, 11, 8].to_vec(),
        ].to_vec();
        log!("{:?}", numbers[0]);
        let safe_sequences: Vec<&Vec<i32>> = numbers.iter()
            .filter(|v| is_safe_with_dampener(v))
            .collect();
        let num_safe = safe_sequences.len();
        for i in 0..2 {
            log!("{:?}", safe_sequences[i]);
        }
        num_safe.to_string()
    }
}
