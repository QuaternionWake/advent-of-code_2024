pub fn run() {
    let input = crate::load_input(7);

    let calibrations: Vec<_> = input
        .lines()
        .map(|l| {
            let chunks: Vec<_> = l.split(':').collect();
            assert_eq!(chunks.len(), 2, "result and operands should be separated by a ':'");
            let result: i64 = chunks[0].parse().expect("could not parse result");
            let operands: Vec<i32> = chunks[1]
                .split(' ')
                .filter_map(|s| s.parse().ok())
                .collect();
            (result, operands)
        })
        .collect();

    let mut calibration_sum = 0;
    for (result, operands) in calibrations {
        if can_be_calibrated(result, &operands) {
            calibration_sum += result;
        }
    }

    println!("Sum of possible calibrations: {calibration_sum}");
}

fn can_be_calibrated(result: i64, operands: &[i32]) -> bool {
    let mut known_adds = 0;
    let mut reverse_sum = 0;
    for operand in operands.iter().rev() {
        if (result - reverse_sum) % *operand as i64 != 0 {
            known_adds += 1;
            reverse_sum += *operand as i64;
        } else {
            break;
        }
    }

    calibrate_rec(result - reverse_sum, 0, &operands[..operands.len() - known_adds])
}

fn calibrate_rec(wanted_res: i64, acc: i64, operands: &[i32]) -> bool {
    if operands.is_empty() {
        acc == wanted_res
    } else if acc > wanted_res {
        false
    } else {
        calibrate_rec(wanted_res, acc + operands[0] as i64, &operands[1..])
            || calibrate_rec(wanted_res, acc * operands[0] as i64, &operands[1..])
    }
}
