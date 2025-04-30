pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
    let jewels: Vec<char> = jewels.chars().collect();

    stones.chars().map(|x| i32::from(jewels.contains(&x))).sum()
}
