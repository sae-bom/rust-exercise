pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
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
            result[item.1] = prev_i.unwrap() as i32;
            continue;
        }
        result[item.1] = i as i32;
        prev_i = Some(i);
        prev_num = Some(item.0);
    }
    result.into_iter().map(|x| x as i32).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = smaller_numbers_than_current(vec![8, 1, 2, 2, 3]);
        assert_eq!(result, vec![4, 0, 1, 1, 3]);
    }

    #[test]
    fn example_2() {
        let result = smaller_numbers_than_current(vec![6, 5, 4, 8]);
        assert_eq!(result, vec![2, 1, 0, 3]);
    }

    #[test]
    fn example_3() {
        let result = smaller_numbers_than_current(vec![7, 7, 7, 7]);
        assert_eq!(result, vec![0, 0, 0, 0]);
    }
}
