use std::iter::zip;

#[allow(dead_code)]
pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Result<Vec<i32>, String> {
    let mut result: Vec<i32> = Vec::with_capacity(nums.len());
    for (num, i) in zip(nums, index) {
        let i: usize = i.try_into().map_err(|e| format!("Invalid index: {e}"))?;

        if i > result.len() {
            return Err(format!(
                "Index {} out of bounds (len = {})",
                i,
                result.len()
            ));
        }
        result.insert(i, num);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = create_target_array(vec![0, 1, 2, 3, 4], vec![0, 1, 2, 2, 1]);
        assert_eq!(result, Ok(vec![0, 4, 1, 3, 2]));
    }

    #[test]
    fn out_of_range_index() {
        let result = create_target_array(vec![0, 1, 2], vec![2, 1, 0]);
        assert!(result.is_err());
    }

    #[test]
    fn not_usize_index() {
        let result = create_target_array(vec![0], vec![-1]);
        assert!(result.is_err());
    }
}
