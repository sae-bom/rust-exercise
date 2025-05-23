#[allow(dead_code)]
pub fn subtract_product_and_sum(n: i32) -> i32 {
    let (product, sum) = n
        .to_string()
        .chars()
        .filter_map(|c| {
            c.to_digit(10).map(|digit| {
                digit.try_into().expect(
                    "A decimal digit is always guaranteed to fit within the range of an i32.",
                )
            })
        })
        .fold((1, 0), |(product, sum), digit: i32| {
            (product * digit, sum + digit)
        });

    product - sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = subtract_product_and_sum(234);
        assert_eq!(result, 15);
    }

    #[test]
    fn returns_minus() {
        let result = subtract_product_and_sum(111);
        assert_eq!(result, -2);
    }
}
