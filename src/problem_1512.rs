use std::collections::HashMap;

#[allow(dead_code)]
pub fn num_identical_pairs(nums: &[i32]) -> i32 {
    let counter: HashMap<i32, i32> = nums.iter().fold(HashMap::new(), |mut ctr, elem| {
        *ctr.entry(*elem).or_insert(0) += 1;
        ctr
    });

    counter.values().map(|val| (val * (val - 1) / 2)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = num_identical_pairs(&[1, 2, 3, 1, 1, 3]);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_2() {
        let result = num_identical_pairs(&[1, 1, 1, 1]);
        assert_eq!(result, 6);
    }

    #[test]
    fn example_3() {
        let result = num_identical_pairs(&[1, 2, 3]);
        assert_eq!(result, 0);
    }
}
