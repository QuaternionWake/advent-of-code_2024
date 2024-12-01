use std::{collections::HashMap, fs, iter};

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
    for (num_1, num_2) in iter::zip(&list_1, &list_2) {
        diff_sum += (num_1 - num_2).abs();
    }

    println!("Total differnce: {diff_sum}");

    // Simpler O(n^2) implementation

    // let mut similarity_score = 0;
    // for x in &list_1 {
    //     for y in &list_2 {
    //         if x == y {
    //             similarity_score += x;
    //         }
    //     }
    // }

    // println!("Similarity score: {similarity_score}");

    let mut set_1 = HashMap::new();
    for val in list_1 {
        set_1
            .entry(val)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut set_2 = HashMap::new();
    for val in list_2 {
        set_2
            .entry(val)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut similarity_score = 0;
    for (value, count) in set_1 {
        let count_2 = *set_2.get(&value).unwrap_or(&0);
        similarity_score += value * count * count_2;
    }

    println!("Similarity score: {similarity_score}");
}
