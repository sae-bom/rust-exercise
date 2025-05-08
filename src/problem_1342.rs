pub fn number_of_steps(num: i32) -> i32 {
    format!("{num:b}")
        .bytes()
        .map(|c| if c == b'0' { 1 } else { 2 })
        .sum::<i32>()
        - 1
}
