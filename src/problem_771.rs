#[allow(dead_code)]
pub fn num_jewels_in_stones(jewels: &str, stones: &str) -> i32 {
    let jewels: Vec<char> = jewels.chars().collect();

    stones.chars().map(|x| i32::from(jewels.contains(&x))).sum()
}
