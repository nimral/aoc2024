fn is_correct(report: Vec<usize>) -> bool {
    let mut last = report[0];
    let mut increasing = true;
    for (i, n) in report[1..].iter().enumerate() {
        if *n == last {
            return false;
        }
        if *n > last {
            if increasing {
                if *n - last > 3 {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            if i == 0 {
                increasing = false;
            }
            if increasing {
                return false;
            } else {
                if last - *n > 3 {
                    return false;
                }
            }
        }
        last = *n;
    }
    return true;
}

fn main() {
    let mut num_correct = 0;
    for line in std::fs::read_to_string("i2.txt").unwrap().lines() {
        let report: Vec<usize> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        num_correct += is_correct(report) as usize;
    }
    println!("{num_correct}");
}
