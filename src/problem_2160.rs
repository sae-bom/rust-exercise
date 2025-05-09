#[allow(dead_code)]
pub fn minimum_sum(num: i32) -> i32 {
    let mut digits = [num / 1000, num / 100 % 10, num / 10 % 10, num % 10];
    digits.sort_unstable();
    (digits[0] + digits[1]) * 10 + digits[2] + digits[3]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = minimum_sum(2932);
        assert_eq!(result, 52);
    }
}
