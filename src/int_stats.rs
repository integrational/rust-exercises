use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::HashMap;

#[derive(Debug)]
pub struct IntStats {
    mean: f64,
    median: f64,
    mode: Option<i32>,
}

pub fn calc_stats(vals: &Vec<i32>) -> IntStats {
    let vals = sorted_clone(vals);

    let (sum, occurences) = collect_sum_and_occurrences(&vals);

    IntStats {
        mean: calc_mean(sum, vals.len()),
        median: calc_median(&vals),
        mode: find_mode(&occurences),
    }
}

fn sorted_clone(vals: &Vec<i32>) -> Vec<i32> {
    let mut vals = vals.clone(); // don't change the original
    vals.sort(); // sort in place
    vals
}

fn collect_sum_and_occurrences(vals: &Vec<i32>) -> (i32, HashMap<i32, u32>) {
    let mut sum = 0;
    let mut occ = HashMap::new(); // key: val, value: count
    for v in vals {
        sum += *v;
        *(occ.entry(*v).or_insert(0)) += 1;
    }
    (sum, occ)
}

fn calc_mean(sum: i32, num: usize) -> f64 {
    (sum as f64) / (num as f64)
}

fn calc_median(vals: &Vec<i32>) -> f64 {
    let num = vals.len();
    if num % 2 == 0 {
        // even
        ((vals[num / 2 - 1] + vals[num / 2]) as f64) / 2.0
    } else {
        // odd
        vals[num / 2] as f64
    }
}

fn find_mode(occurences: &HashMap<i32, u32>) -> Option<i32> {
    let mut mode = None;
    let mut max_count = 0;
    for (v, count) in occurences {
        match max_count.cmp(count) {
            Less => {
                mode = Some(*v); // new mode candidate
                max_count = *count;
            }
            Equal => mode = None, // no mode if not a single most frequent value
            Greater => (),
        }
    }
    mode
}
