#[allow(dead_code)]
pub fn count_matches(
    items: &[&[&str]],
    rule_key: &str,
    rule_value: &str,
) -> Result<usize, &'static str> {
    items.iter().try_fold(0, |acc, item| {
        let [type_, color, name] = (*item)
            .try_into()
            .map_err(|_| "Each item should have exactly 3 fields.")?;

        let value = match rule_key {
            "type" => type_,
            "color" => color,
            "name" => name,
            _ => return Err("Unknown rule_key given."),
        };

        Ok(acc + usize::from(value == rule_value))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = count_matches(
            &[
                &["phone", "blue", "pixel"],
                &["computer", "silver", "lenovo"],
                &["phone", "gold", "iphone"],
            ],
            "color",
            "silver",
        );
        assert_eq!(result, Ok(1));
    }

    #[test]
    fn example_2() {
        let result = count_matches(
            &[
                &["phone", "blue", "pixel"],
                &["computer", "silver", "lenovo"],
                &["phone", "gold", "iphone"],
            ],
            "type",
            "phone",
        );
        assert_eq!(result, Ok(2));
    }

    #[test]
    fn invalid_item() {
        let result = count_matches(&[&["phone", "blue", "pixel", "melon"]], "type", "blah");
        assert!(result.is_err());
    }

    #[test]
    fn invalid_rule_key() {
        let result = count_matches(&[&["phone", "blue", "pixel"]], "unknown_rule_key", "blah");
        assert!(result.is_err());
    }
}
