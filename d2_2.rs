fn is_correct(report: &Vec<usize>, skip_index: usize) -> bool {
    println!("{report:?} {skip_index}");
    let mut last_set = false;
    let mut last = 0;
    let mut increasing = true;
    let mut num_seen = 0;
    for (i, n) in report.iter().enumerate() {
        if i == skip_index {
            continue;
        }
        num_seen += 1;
        if !last_set {
            last = *n;
            last_set = true;
            continue;
        }
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
            if num_seen == 2 {
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
        let mut can_be_correct = false;
        for i in 0..report.len() {
            if is_correct(&report, i) {
                can_be_correct = true;
                break;
            }
        }
        println!();
        num_correct += can_be_correct as usize;
    }
    println!("{num_correct}");
}
