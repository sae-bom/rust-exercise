#[allow(dead_code)]
pub fn running_sum(mut nums: Vec<i32>) -> Vec<i32> {
    for idx in 1..nums.len() {
        nums[idx] += nums[idx - 1];
    }
    nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = running_sum(vec![1, 2, 3, 4]);
        assert_eq!(result, vec![1, 3, 6, 10]);
    }

    #[test]
    fn example_2() {
        let result = running_sum(vec![1, 1, 1, 1, 1]);
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn example_3() {
        let result = running_sum(vec![3, 1, 2, 10, 1]);
        assert_eq!(result, vec![3, 4, 6, 16, 17]);
    }
}
