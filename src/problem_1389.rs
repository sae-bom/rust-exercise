use std::iter::zip;

pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::with_capacity(nums.len());
    for (num, i) in zip(nums, index) {
        result.insert(i.try_into().expect("i must be non-negative"), num);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = create_target_array(vec![0, 1, 2, 3, 4], vec![0, 1, 2, 2, 1]);
        assert_eq!(result, vec![0, 4, 1, 3, 2]);
    }
}
