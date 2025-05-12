use std::collections::BTreeMap;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn smaller_numbers_than_current(nums: &[i32]) -> Vec<i32> {
    let num_counter = nums.iter().fold(BTreeMap::new(), |mut ctr, num| {
        *ctr.entry(*num).or_insert(0) += 1;
        ctr
    });

    let num_cumulation: HashMap<i32, i32> = num_counter
        .into_iter()
        .scan(0, |cumulation, (n, count)| {
            *cumulation += count;
            Some((n, *cumulation - count))
        })
        .collect();

    nums.iter()
        .map(|n| *num_cumulation.get(n).expect("Key 'n' must be in the map"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = smaller_numbers_than_current(&[8, 1, 2, 2, 3]);
        assert_eq!(result, vec![4, 0, 1, 1, 3]);
    }

    #[test]
    fn example_2() {
        let result = smaller_numbers_than_current(&[6, 5, 4, 8]);
        assert_eq!(result, vec![2, 1, 0, 3]);
    }

    #[test]
    fn example_3() {
        let result = smaller_numbers_than_current(&[7, 7, 7, 7]);
        assert_eq!(result, vec![0, 0, 0, 0]);
    }
}
