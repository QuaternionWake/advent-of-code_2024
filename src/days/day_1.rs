use std::{fs, iter};

pub fn run() {
    let input_path = "inputs/day_1.txt";
    let input = fs::read_to_string(input_path)
        .expect(format!("couldn't read input file \"{input_path}\"").as_str());

    let mut list_1 = vec![];
    let mut list_2 = vec![];
    for line in input.lines() {
        let nums: Vec<i32> = line.split(' ').filter_map(|s| s.parse().ok()).collect();
        assert_eq!(nums.len(), 2, "each line should contain 2 numbers");
        list_1.push(nums[0]);
        list_2.push(nums[1]);
    }

    list_1.sort();
    list_2.sort();

    let mut diff_sum = 0;
    for (num_1, num_2) in iter::zip(list_1.iter(), list_2.iter()) {
        diff_sum += (num_1 - num_2).abs();
    }

    println!("{diff_sum}");
}
