use std::collections::HashMap;

pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    let mut counter: HashMap<i32, u16> = HashMap::new();

    for elem in nums.iter() {
        counter.entry(*elem).and_modify(|e| *e += 1).or_insert(1);
    }

    let mut result = 0;

    for val in counter.values().filter(|&x| *x >= 2) {
        result += val * (val - 1) / 2;
    }

    result as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = num_identical_pairs(vec![1, 2, 3, 1, 1, 3]);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_2() {
        let result = num_identical_pairs(vec![1, 1, 1, 1]);
        assert_eq!(result, 6);
    }

    #[test]
    fn example_3() {
        let result = num_identical_pairs(vec![1, 2, 3]);
        assert_eq!(result, 0);
    }
}
