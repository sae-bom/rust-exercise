#[allow(dead_code)]
pub fn minimum_sum(num: u32) -> u32 {
    let mut digits: Vec<u32> = num
        .to_string()
        .chars()
        .map(|c| {
            c.to_digit(10)
                .expect("A decimal digit is always guaranteed to fit within the range of an u32.")
        })
        .collect();

    digits.sort_unstable_by(|a, b| b.cmp(a));

    digits
        .chunks(2)
        .fold((0, 1), |(result, significant), d| {
            (
                result + d.iter().sum::<u32>() * significant,
                significant * 10,
            )
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = minimum_sum(2932);
        assert_eq!(result, 52);
    }

    #[test]
    fn odd_digit_number() {
        let result = minimum_sum(987);
        assert_eq!(result, 78 + 9);
    }

    #[test]
    fn zero() {
        let result = minimum_sum(0);
        assert_eq!(result, 0);
    }
}
