use std::fs;

pub fn run() {
    let input_path = "inputs/day_2.txt";
    let input = fs::read_to_string(input_path)
        .expect(format!("couldn't read input file \"{input_path}\"").as_str());

    let mut reports = vec![];
    for line in input.lines() {
        let nums: Vec<i32> = line
            .split(' ')
            .map(|s| s.parse().expect("input should only contain numbers"))
            .collect();
        reports.push(nums);
    }

    let mut safe_count = 0;
    for report in reports {
        if report.len() < 2 {
            safe_count += 1;
            continue;
        }

        match report[0].cmp(&report[1]) {
            std::cmp::Ordering::Equal => continue,
            std::cmp::Ordering::Less => {
                if report.windows(2).all(|x| (1..=3).contains(&(x[1] - x[0]))) {
                    safe_count += 1;
                }
            }
            std::cmp::Ordering::Greater => {
                if report.windows(2).all(|x| (1..=3).contains(&(x[0] - x[1]))) {
                    safe_count += 1;
                }
            }
        }
    }

    println!("Safe report count: {safe_count}");
}
