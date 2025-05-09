#[allow(dead_code)]
pub fn build_array(mut nums: Vec<i32>) -> Vec<i32> {
    let n: usize = nums.len();
    let n_i32: i32 = n.try_into().expect("1 <= nums.length <= 1000");
    let mut loop_start_idx: usize = 0;

    while loop_start_idx < n {
        if nums[loop_start_idx] >= n_i32 {
            loop_start_idx += 1;
            continue;
        }
        let &loop_start_num = nums.get(loop_start_idx).expect("Index out of bound");

        let mut idx = loop_start_idx;
        loop {
            if loop_start_idx == nums[idx].try_into().unwrap() {
                nums[idx] = loop_start_num + n_i32;
                break;
            }
            let prev_num = nums[idx].try_into().unwrap();
            nums[idx] = nums[prev_num] + n_i32;
            idx = prev_num;
        }
        loop_start_idx += 1;
    }
    for x in &mut nums {
        *x -= n_i32;
    }
    nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = build_array(vec![5, 0, 1, 2, 3, 4]);
        assert_eq!(result, vec![4, 5, 0, 1, 2, 3]);
    }
}
