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
    for (result, operands) in &calibrations {
        if can_be_calibrated(*result, operands) {
            calibration_sum += result;
        }
    }

    println!("Sum of possible calibrations: {calibration_sum}");

    let mut calibration_sum_with_cat = 0;
    for (result, operands) in &calibrations {
        if can_be_calibrated_with_cat(*result, operands) {
            calibration_sum_with_cat += result;
        }
    }

    println!("Sum of possible calibrations with concatenation: {calibration_sum_with_cat}");
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

fn can_be_calibrated_with_cat(result: i64, operands: &[i32]) -> bool {
    calibrate_rec_with_cat(result, 0, operands)
}

fn calibrate_rec_with_cat(wanted_res: i64, acc: i64, operands: &[i32]) -> bool {
    if operands.is_empty() {
        acc == wanted_res
    } else if acc > wanted_res {
        false
    } else {
        let add_acc = acc + operands[0] as i64;
        let mul_acc = acc * operands[0] as i64;
        let cat_acc = if operands[0] == 0 {
            acc * 10
        } else {
            let log = operands[0].ilog10() + 1;
            acc * 10_i64.pow(log) + operands[0] as i64
        };
        calibrate_rec_with_cat(wanted_res, add_acc, &operands[1..])
            || calibrate_rec_with_cat(wanted_res, mul_acc, &operands[1..])
            || calibrate_rec_with_cat(wanted_res, cat_acc, &operands[1..])
    }
}
