use std::collections::HashSet;

#[allow(dead_code)]
pub fn num_jewels_in_stones(jewels: &str, stones: &str) -> usize {
    let jewels: HashSet<char> = jewels.chars().collect();

    stones.chars().filter(|x| jewels.contains(x)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = num_jewels_in_stones("aA", "aAAbbbb");
        assert_eq!(result, 3);
    }

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
