pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let n: usize = n.try_into().unwrap();
    nums[..n]
        .iter()
        .zip(&nums[n..])
        .flat_map(|(&x, &y)| [x, y])
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = shuffle(vec![2, 5, 1, 3, 4, 7], 3);
        assert_eq!(result, vec![2, 3, 5, 4, 1, 7]);
    }
}
