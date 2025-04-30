pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
    operations
        .iter()
        .map(|s| {
            if s.chars().nth(1).unwrap() == '-' {
                -1
            } else {
                1
            }
        })
        .sum()
}
