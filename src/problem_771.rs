#[allow(dead_code)]
pub fn num_jewels_in_stones(jewels: &str, stones: &str) -> i32 {
    let jewels: Vec<char> = jewels.chars().collect();

    stones.chars().map(|x| i32::from(jewels.contains(&x))).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_jewels() {
        let result = num_jewels_in_stones("", "abc");
        assert_eq!(result, 0);
    }

    #[test]
    fn empty_stones() {
        let result = num_jewels_in_stones("bc", "");
        assert_eq!(result, 0);
    }
}
