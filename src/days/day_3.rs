use std::fs;

pub fn run() {
    let input_path = "inputs/day_3.txt";
    let input = fs::read_to_string(input_path)
        .expect(format!("couldn't read input file \"{input_path}\"").as_str());

    let mut potential_starts = vec![];
    let chars: Vec<char> = input.chars().collect();
    for (i, chunk) in chars.windows(3).enumerate() {
        if chunk == ['m', 'u', 'l'] {
            potential_starts.push(i + 4);
        }
    }

    let mut potential_slices = vec![];
    for i in potential_starts {
        if chars[i - 1] != '(' {
            continue;
        }
        let potential_end = chars[i..]
            .iter()
            .enumerate()
            .find(|&x| !(x.1.is_ascii_digit() || *x.1 == ','));
        if let Some(end) = potential_end {
            if *end.1 == ')' {
                potential_slices.push(&chars[i..i + end.0]);
                // for char in chars[i..i + end.0].iter() {
                //     eprint!("{}", char);
                // }
                // println!();
            }
        }
    }

    let mut sum = 0;
    for slice in potential_slices {
        let str: String = slice.iter().collect();
        let args: Vec<_> = str.split(',').collect();
        if args.len() != 2 || args[0].len() > 3 || args[1].len() > 3 {
            continue;
        }
        if let (Ok(arg1), Ok(arg2)) = (args[0].parse::<i32>(), args[1].parse::<i32>()) {
            sum += arg1 * arg2;
        }
    }

    println!("{sum}");
}
