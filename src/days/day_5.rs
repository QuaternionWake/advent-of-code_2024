use std::fs;

pub fn run() {
    let input_path = "inputs/day_5.txt";
    let input = fs::read_to_string(input_path)
        .expect(format!("couldn't read input file \"{input_path}\"").as_str());

    let input: Vec<_> = input.split("\n\n").collect();
    assert_eq!(input.len(), 2, "input should contain rules and updates separated by an empty line");

    let rules: Vec<_> = input[0]
        .lines()
        .map(|s| {
            let nums: Vec<i32> = s.split('|').filter_map(|s| s.parse().ok()).collect();
            assert_eq!(nums.len(), 2, "each rule should be two numbers separated by a '|'");
            (nums[0], nums[1])
        })
        .collect();

    let updates: Vec<Vec<i32>> = input[1]
        .lines()
        .map(|s| s.split(',').filter_map(|s| s.parse().ok()).collect())
        .collect();

    let ordered_updates = updates.iter().filter(|u| is_ordered(u, &rules));
    let mid_sum = ordered_updates.fold(0, |acc, x| acc + x[x.len() / 2]);

    println!("Sum of middle pages: {mid_sum}");
}

fn is_ordered(update: &Vec<i32>, rules: &Vec<(i32, i32)>) -> bool {
    for i in 0..update.len() {
        for j in i + 1..update.len() {
            if rules.contains(&(update[j], update[i])) {
                return false;
            }
        }
    }

    true
}
