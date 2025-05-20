#[allow(dead_code)]
pub fn shuffle(nums: &[i32]) -> Result<Vec<i32>, &str> {
    if nums.is_empty() || nums.len() > 500 || nums.len() % 2 != 0 {
        return Err("Invalid input: nums.len() must be even and in the range 1 to 500 inclusive");
    }

    let n: usize = nums.len() / 2;
    Ok(nums
        .get(..n)
        .expect("1 <= n < nums.length")
        .iter()
        .zip(nums.get(n..).expect("1 <= n < nums.length"))
        .flat_map(|(&x, &y)| [x, y])
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = shuffle(&[2, 5, 1, 3, 4, 7]);
        assert_eq!(result, Ok(vec![2, 3, 5, 4, 1, 7]));
    }
}
