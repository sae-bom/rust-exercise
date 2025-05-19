#[allow(dead_code)]
pub fn build_array(nums: &[usize]) -> Result<Vec<usize>, String> {
    let len = nums.len();
    nums.iter()
        .map(|&n| {
            if n >= len {
                return Err(format!("Invalid index: {n}"));
            }
            Ok(*nums.get(n).expect("0 <= n < nums.length"))
        })
        .collect::<Result<Vec<usize>, String>>()
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
