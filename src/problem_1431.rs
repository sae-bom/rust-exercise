#[allow(dead_code)]
pub fn kids_with_candies(candies: &[i32], extra_candies: i32) -> Vec<bool> {
    let Some(max_candy) = candies.iter().max() else {
        return vec![];
    };
    let at_least_candy = max_candy - extra_candies;

    candies.iter().map(|&x| x >= at_least_candy).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = kids_with_candies(&[4, 2, 1, 1, 2], 1);
        assert_eq!(result, vec![true, false, false, false, false]);
    }

    #[test]
    fn one_kid() {
        let result = kids_with_candies(&[4], 1);
        assert_eq!(result, vec![true]);
    }

    #[test]
    fn no_kid() {
        let result = kids_with_candies(&[], 1);
        assert_eq!(result, vec![]);
    }
}
