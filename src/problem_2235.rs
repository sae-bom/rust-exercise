pub fn sum(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(sum(-10, 4), -6);
    }

    #[test]
    fn test_sum_upper_bound() {
        assert_eq!(sum(100, 100), 200);
    }

    #[test]
    fn test_sum_lower_bound() {
        assert_eq!(sum(-100, -100), -200);
    }
}
