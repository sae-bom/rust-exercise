#[allow(dead_code)]
pub fn shuffle(nums: &[i32]) -> Result<Vec<i32>, &'static str> {
    if nums.len() % 2 != 0 {
        return Err("Invalid input: nums.len() must be even");
    }

    let (left, right) = nums.split_at(nums.len() / 2);
    Ok(left
        .iter()
        .zip(right.iter())
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

    #[test]
    fn empty_input() {
        let result = shuffle(&[]);
        assert_eq!(result, Ok(vec![]));
    }
}
