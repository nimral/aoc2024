fn main() {
    let mut left_list = Vec::<i32>::new();
    let mut right_list = Vec::<i32>::new();
    for line in std::fs::read_to_string("i1.txt").unwrap().lines() {
        let mut num_str_it = line.split_whitespace();
        left_list.push(num_str_it.next().unwrap().parse().unwrap());
        right_list.push(num_str_it.next().unwrap().parse().unwrap());
    }
    left_list.sort();
    right_list.sort();
    let sum: i32 = left_list
        .into_iter()
        .zip(right_list.into_iter())
        .map(|(x, y)| (x - y).abs())
        .sum();
    println!("{sum}");
}
