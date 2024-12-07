use std::cmp;

pub fn run() {
    let input = crate::load_input(5);

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

    let mut updates: Vec<Vec<i32>> = input[1]
        .lines()
        .map(|s| s.split(',').filter_map(|s| s.parse().ok()).collect())
        .collect();

    let ordered_updates = updates.iter().filter(|u| is_ordered(u, &rules));
    let mid_sum = ordered_updates.fold(0, |acc, x| acc + x[x.len() / 2]);

    println!("Sum of middle pages: {mid_sum}");

    let unordered_updates = updates
        .iter_mut()
        .filter(|u| !is_ordered(u, &rules))
        .map(|u| {
            u.sort_by(|&lhs, &rhs| compare_pages(&rules, lhs, rhs));
            u
        });

    let mid_sum = unordered_updates.fold(0, |acc, x| acc + x[x.len() / 2]);

    println!("Sum of remaining pages: {mid_sum}");
}

fn is_ordered(update: &[i32], rules: &[(i32, i32)]) -> bool {
    for i in 0..update.len() {
        for j in i + 1..update.len() {
            if rules.contains(&(update[j], update[i])) {
                return false;
            }
        }
    }

    true
}

fn compare_pages(rules: &[(i32, i32)], lhs: i32, rhs: i32) -> cmp::Ordering {
    if rules.contains(&(lhs, rhs)) {
        cmp::Ordering::Less
    } else if rules.contains(&(rhs, lhs)) {
        cmp::Ordering::Greater
    } else {
        cmp::Ordering::Equal
    }
}
