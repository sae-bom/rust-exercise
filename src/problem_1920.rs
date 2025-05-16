#[allow(dead_code)]
pub fn build_array(nums: &[i32]) -> Vec<i32> {
    nums.iter()
        .map(|&n| {
            let n: usize = n.try_into().expect("0 <= nums[i] < nums.length");
            *nums.get(n).expect("0 <= nums[i] < nums.length")
        })
        .collect::<Vec<i32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = build_array(&[5, 0, 1, 2, 3, 4]);
        assert_eq!(result, vec![4, 5, 0, 1, 2, 3]);
    }
}
