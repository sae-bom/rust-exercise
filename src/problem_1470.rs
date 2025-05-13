#[allow(dead_code)]
pub fn shuffle(nums: &[i32], n: i32) -> Vec<i32> {
    let n: usize = n
        .try_into()
        .expect("n is guaranteed to be within `usize` by the problem description");
    nums.get(..n)
        .expect("n < nums.length, by the problem description")
        .iter()
        .zip(
            nums.get(n..)
                .expect("n < nums.length, by the problem description"),
        )
        .flat_map(|(&x, &y)| [x, y])
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = shuffle(&[2, 5, 1, 3, 4, 7], 3);
        assert_eq!(result, vec![2, 3, 5, 4, 1, 7]);
    }
}
