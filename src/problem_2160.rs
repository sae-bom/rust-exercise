#[allow(dead_code)]
pub fn minimum_sum(num: i32) -> i32 {
    let mut digits = [num / 1000, num / 100 % 10, num / 10 % 10, num % 10];
    digits.sort_unstable();
    let [
        smaller_digit_1,
        smaller_digit_2,
        bigger_digit_1,
        bigger_digit_2,
    ] = &digits;
    (smaller_digit_1 + smaller_digit_2) * 10 + bigger_digit_1 + bigger_digit_2
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
