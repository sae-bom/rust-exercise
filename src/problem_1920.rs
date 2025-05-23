#[allow(dead_code)]
pub fn build_array(nums: &[usize]) -> Result<Vec<usize>, String> {
    nums.iter()
        .map(|&n| {
            let num = nums.get(n).ok_or_else(|| format!("Invalid index: {n}"))?;
            Ok(*num)
        })
        .collect::<Result<Vec<_>, String>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = build_array(&[5, 0, 1, 2, 3, 4]);
        assert_eq!(result, Ok(vec![4, 5, 0, 1, 2, 3]));
    }

    #[test]
    fn invalid_index() {
        let result = build_array(&[0, 2]);
        assert!(result.is_err());
    }
}
