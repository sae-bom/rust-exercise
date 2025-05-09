#[allow(dead_code)]
pub fn smaller_numbers_than_current(nums: &[i32]) -> Vec<i32> {
    let mut num_idx_pair = nums
        .iter()
        .enumerate()
        .map(|(idx, &num)| (num, idx))
        .collect::<Vec<(i32, usize)>>();

    num_idx_pair.sort_unstable_by_key(|&(x, _)| x);

    let mut prev_num: Option<i32> = None;
    let mut prev_i: Option<usize> = None;
    let mut result = vec![0; nums.len()];
    for (i, item) in num_idx_pair.iter().enumerate() {
        if prev_num.is_some() && prev_num.unwrap() == item.0 {
            result[item.1] = prev_i.unwrap().try_into().unwrap();
            continue;
        }
        result[item.1] = i.try_into().unwrap();
        prev_i = Some(i);
        prev_num = Some(item.0);
    }
    result
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
