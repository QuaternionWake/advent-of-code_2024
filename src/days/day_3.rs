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
                potential_slices.push((i, &chars[i..i + end.0]));
                // for char in chars[i..i + end.0].iter() {
                //     eprint!("{}", char);
                // }
                // println!();
            }
        }
    }

    let mut valid_muls = vec![];
    for (i, slice) in potential_slices {
        let str: String = slice.iter().collect();
        let args: Vec<_> = str.split(',').collect();
        if args.len() != 2 || args[0].len() > 3 || args[1].len() > 3 {
            continue;
        }
        if let (Ok(arg1), Ok(arg2)) = (args[0].parse::<i32>(), args[1].parse::<i32>()) {
            valid_muls.push((i, arg1 * arg2));
        }
    }
    let sum: i32 = valid_muls.iter().map(|x| x.1).sum();

    println!("Sum of multiples: {sum}");

    let mut funcs = vec![];
    for (i, chunk) in chars.windows(4).enumerate() {
        if chunk == ['d', 'o', '(', ')'] {
            funcs.push((i, Func::Enable));
        }
    }
    for (i, chunk) in chars.windows(7).enumerate() {
        if chunk == ['d', 'o', 'n', '\'', 't', '(', ')'] {
            funcs.push((i, Func::Disable));
        }
    }
    for (i, mul) in valid_muls {
        funcs.push((i, Func::Multiply(mul)));
    }

    funcs.sort_by(|x, y| x.0.cmp(&y.0));

    let mut sum = 0;
    let mut enabled = true;
    for (_, func) in funcs {
        match func {
            Func::Multiply(n) => {
                if enabled {
                    sum += n
                }
            }
            Func::Enable => enabled = true,
            Func::Disable => enabled = false,
        }
    }

    println!("Sum with conditionals: {sum}")
}

enum Func {
    Multiply(i32),
    Enable,
    Disable,
}
