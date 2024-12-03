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
    for report in &reports {
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

    // Simpler O(n*m^2) solution

    // let mut safe_count = 0;
    // for report in &reports {
    //     for i in 0..report.len() {
    //         let mut report = report.clone();
    //         report.remove(i);

    //         if report.len() < 2 {
    //             safe_count += 1;
    //             break;
    //         }

    //         match report[0].cmp(&report[1]) {
    //             std::cmp::Ordering::Equal => continue,
    //             std::cmp::Ordering::Less => {
    //                 if report.windows(2).all(|x| (1..=3).contains(&(x[1] - x[0]))) {
    //                     safe_count += 1;
    //                     break;
    //                 }
    //             }
    //             std::cmp::Ordering::Greater => {
    //                 if report.windows(2).all(|x| (1..=3).contains(&(x[0] - x[1]))) {
    //                     safe_count += 1;
    //                     break;
    //                 }
    //             }
    //         }
    //     }
    // }

    // println!("Safe report with dampener count: {safe_count}");

    let mut safe_count = 0;
    for report in &reports {
        if report.len() < 3 {
            safe_count += 1;
            continue;
        }

        let mut is_safe = true;
        let mut dampener_available = true;
        let mut skip_next = false;
        if !(1..=3).contains(&(report[0] - report[1])) {
            if (1..=3).contains(&(report[0] - report[2])) {
                dampener_available = false;
                skip_next = true;
            } else {
                dampener_available = false;
            }
        }
        for chunk in report.windows(4) {
            if skip_next {
                skip_next = false;
                continue;
            }
            if (1..=3).contains(&(chunk[1] - chunk[2])) {
                continue;
            }
            if !dampener_available {
                is_safe = false;
                break;
            }
            if (1..=3).contains(&(chunk[1] - chunk[3])) {
                dampener_available = false;
                skip_next = true;
            } else if (1..=3).contains(&(chunk[0] - chunk[2])) {
                dampener_available = false;
            } else {
                is_safe = false;
                break;
            }
        }
        if !(1..=3).contains(&(report[report.len() - 2] - report[report.len() - 1]))
            && !skip_next
            && !dampener_available
        {
            is_safe = false;
        }
        if is_safe {
            safe_count += 1;
            continue;
        }

        let mut is_safe = true;
        let mut dampener_available = true;
        let mut skip_next = false;
        if !(1..=3).contains(&(report[1] - report[0])) {
            if (1..=3).contains(&(report[2] - report[0])) {
                dampener_available = false;
                skip_next = true;
            } else {
                dampener_available = false;
            }
        }
        for chunk in report.windows(4) {
            if skip_next {
                skip_next = false;
                continue;
            }
            if (1..=3).contains(&(chunk[2] - chunk[1])) {
                continue;
            }
            if !dampener_available {
                is_safe = false;
                break;
            }
            if (1..=3).contains(&(chunk[3] - chunk[1])) {
                dampener_available = false;
                skip_next = true;
            } else if (1..=3).contains(&(chunk[2] - chunk[0])) {
                dampener_available = false;
            } else {
                is_safe = false;
                break;
            }
        }
        if !(1..=3).contains(&(report[report.len() - 1] - report[report.len() - 2]))
            && !skip_next
            && !dampener_available
        {
            is_safe = false;
        }
        if is_safe {
            safe_count += 1;
        }
    }

    println!("Safe report with dampener count: {safe_count}");
}
