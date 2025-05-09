pub fn sort_sentence(s: String) -> String {
    let mut sorted_words = s.split_whitespace().collect::<Vec<&str>>();
    sorted_words.sort_unstable_by_key(|str| str.chars().last());

    sorted_words
        .iter()
        .map(|&str| &str[..str.len() - 1])
        .collect::<Vec<&str>>()
        .join(" ")
}
