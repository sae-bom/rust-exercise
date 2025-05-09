#[allow(dead_code)]
pub fn subtract_product_and_sum(mut n: i32) -> i32 {
    let mut digits = Vec::new();
    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }

    digits.iter().product::<i32>() - digits.iter().sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = subtract_product_and_sum(234);
        assert_eq!(result, 15);
    }
}
