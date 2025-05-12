#[allow(dead_code)]
pub fn subtract_product_and_sum(n: i32) -> i32 {
    let (product, sum) = std::iter::successors(Some((n, None)), |(n, _)| {
        if *n == 0 {
            return None;
        }
        let digit = n % 10;
        Some((n / 10, Some(digit)))
    })
    .filter_map(|(_, digit)| digit)
    .fold((1, 0), |(product, sum), digit| {
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
}
