#[allow(dead_code)]
pub fn maximum_wealth(accounts: &[Vec<i32>]) -> i32 {
    accounts.iter().map(|x| x.iter().sum()).max().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = maximum_wealth(&[vec![1, 2, 3], vec![3, 2, 1]]);
        assert_eq!(result, 6);
    }

    #[test]
    fn example_2() {
        let result = maximum_wealth(&[vec![1, 5], vec![7, 3], vec![3, 5]]);
        assert_eq!(result, 10);
    }

    #[test]
    fn example_3() {
        let result = maximum_wealth(&[vec![2, 8, 7], vec![7, 1, 3], vec![1, 9, 5]]);
        assert_eq!(result, 17);
    }

    #[test]
    fn no_customer() {
        let result = maximum_wealth(&[]);
        assert_eq!(result, 0);
    }
}
