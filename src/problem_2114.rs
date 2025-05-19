#[allow(dead_code)]
pub fn most_words_found(sentences: &[String]) -> usize {
    sentences
        .iter()
        .map(|sentence| sentence.split(' ').collect::<Vec<&str>>().len())
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = most_words_found(&[
            "alice and bob love leetcode".to_string(),
            "i think so too".to_string(),
            "this is great thanks very much".to_string(),
        ]);
        assert_eq!(result, 6);
    }

    #[test]
    fn no_input() {
        let result = most_words_found(&[]);
        assert_eq!(result, 0);
    }
}
