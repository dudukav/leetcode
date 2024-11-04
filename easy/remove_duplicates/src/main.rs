pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut count_unique: i32 = 0;
    let mut tmp_vec: Vec<i32> = Vec::new();
    let mut curr_num = nums[0];
    for num in nums[1..].iter() {
        if curr_num != *num {
            count_unique += 1;
            tmp_vec.push(curr_num);
            curr_num = *num;
        }
    }

    count_unique += 1;
    tmp_vec.push(curr_num);

    nums.clear();
    nums.extend(tmp_vec.iter().cloned());

    count_unique
}

fn main() {
    println!("Hello, world!");
}
