use std::collections::HashMap;

fn main() {
    let mut left_list = Vec::<usize>::new();
    let mut right_counts = HashMap::<usize, usize>::new();
    for line in std::fs::read_to_string("i1.txt").unwrap().lines() {
        let mut num_str_it = line.split_whitespace();
        left_list.push(num_str_it.next().unwrap().parse().unwrap());
        *right_counts
            .entry(num_str_it.next().unwrap().parse().unwrap())
            .or_default() += 1;
    }

    let sum: usize = left_list
        .into_iter()
        .map(|x| {
            x * match right_counts.get(&x) {
                Some(freq) => freq,
                None => &0,
            }
        })
        .sum();
    println!("{sum}");
}
