#[allow(dead_code)]
pub fn count_matches(
    items: Vec<Vec<String>>,
    rule_key: &str,
    rule_value: &str,
) -> Result<usize, String> {
    Ok(items
        .into_iter()
        .map(|item| {
            let [type_, color, name] = item
                .try_into()
                .map_err(|_| "Each item should have exactly 3 fields.")?;

            let value = match rule_key {
                "type" => type_,
                "color" => color,
                "name" => name,
                _ => return Err("Unknown rule_key given."),
            };

            Ok(usize::from(value == rule_value))
        })
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .sum::<usize>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = count_matches(
            vec![
                vec!["phone".to_string(), "blue".to_string(), "pixel".to_string()],
                vec![
                    "computer".to_string(),
                    "silver".to_string(),
                    "lenovo".to_string(),
                ],
                vec![
                    "phone".to_string(),
                    "gold".to_string(),
                    "iphone".to_string(),
                ],
            ],
            "color",
            "silver",
        );
        assert_eq!(result, Ok(1));
    }

    #[test]
    fn example_2() {
        let result = count_matches(
            vec![
                vec!["phone".to_string(), "blue".to_string(), "pixel".to_string()],
                vec![
                    "computer".to_string(),
                    "silver".to_string(),
                    "lenovo".to_string(),
                ],
                vec![
                    "phone".to_string(),
                    "gold".to_string(),
                    "iphone".to_string(),
                ],
            ],
            "type",
            "phone",
        );
        assert_eq!(result, Ok(2));
    }

    #[test]
    fn invalid_item() {
        let result = count_matches(
            vec![vec![
                "phone".to_string(),
                "blue".to_string(),
                "pixel".to_string(),
                "melon".to_string(),
            ]],
            "type",
            "blah",
        );
        assert!(result.is_err());
    }

    #[test]
    fn invalid_rule_key() {
        let result = count_matches(
            vec![vec![
                "phone".to_string(),
                "blue".to_string(),
                "pixel".to_string(),
            ]],
            "unknown_rule_key",
            "blah",
        );
        assert!(result.is_err());
    }
}
