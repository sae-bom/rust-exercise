#[allow(dead_code)]
pub fn final_value_after_operations(operations: &[String]) -> i32 {
    operations
        .iter()
        .map(|s| {
            if s.len() == 3 {
                let sign = s
                    .chars()
                    .nth(1)
                    .expect("s.len() == 3 is already guaranteed.");
                match sign {
                    '-' => -1,
                    '+' => 1,
                    _ => 0,
                }
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = final_value_after_operations(&[
            "--X".to_string(),
            "X++".to_string(),
            "X++".to_string(),
        ]);
        assert_eq!(result, 1);
    }
}
