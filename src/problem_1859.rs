#[allow(dead_code)]
pub fn sort_sentence(s: &str) -> String {
    if s.len() <= 1 {
        return String::new();
    }
    let mut sorted_words = s.split_whitespace().collect::<Vec<&str>>();
    sorted_words.sort_unstable_by_key(|str| str.chars().last());

    sorted_words
        .iter()
        .map(|&str| {
            str.get(..str.len() - 1).expect(
                "It is always within the valid range, given that s.length >= 2 is guaranteed.",
            )
        })
        .collect::<Vec<&str>>()
        .join(" ")
}
