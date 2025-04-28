pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let max_candy = candies.iter().max().expect("candies.length >= 2");
    let at_least_candy = max_candy - extra_candies;

    candies.iter().map(|&x| x >= at_least_candy).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = kids_with_candies(vec![4, 2, 1, 1, 2], 1);
        assert_eq!(result, vec![true, false, false, false, false]);
    }
}
