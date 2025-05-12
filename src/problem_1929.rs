#[allow(dead_code)]
pub fn get_concatenation(nums: &[i32]) -> Vec<i32> {
    [nums, nums].concat()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(get_concatenation(&[1, 2, 1]), &[1, 2, 1, 1, 2, 1]);
    }
}
