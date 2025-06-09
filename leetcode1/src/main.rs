fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in nums.iter().enumerate() {
        for j in (i.0 + 1)..nums.len() {
            if i.1 + nums[j] == target {
                return vec![i.0 as i32, j as i32];
            }
        }
    }
    vec![]
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = two_sum(nums, target);
    println!("{:?}", result); // Output: [0, 1]

    let nums2 = vec![3, 2, 4];
    let target2 = 6;
    let result2 = two_sum(nums2, target2);
    println!("{:?}", result2); // Output: [1, 2]

    let nums3 = vec![3, 3];
    let target3 = 6;
    let result3 = two_sum(nums3, target3);
    println!("{:?}", result3); // Output: [0, 1]
}
