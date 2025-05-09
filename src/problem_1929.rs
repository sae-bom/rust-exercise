pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    let mut result: Vec<i32> = vec![0; len * 2];

    result[..len].clone_from_slice(&nums[..]);
    result[len..2 * len].clone_from_slice(&nums[..]);
    result
}
