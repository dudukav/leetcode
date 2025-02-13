use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut nums_to_index = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;

        if let Some(&index) = nums_to_index.get(&complement) {
            return vec![index as i32, i as i32];
        }

        nums_to_index.insert(num, i);
    }

    vec![]
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = two_sum(nums, target);

    println!("{:?}", result);
}