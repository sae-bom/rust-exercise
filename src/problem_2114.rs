#[allow(dead_code)]
pub fn most_words_found(sentences: &[String]) -> i32 {
    sentences
        .iter()
        .map(|sentence| sentence.split_whitespace().count())
        .max()
        .expect("Parameter 'sentences' is not empty")
        .try_into()
        .expect("Length of sentences do not exceed i32 range")
}
