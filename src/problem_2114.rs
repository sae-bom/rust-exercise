pub fn most_words_found(sentences: Vec<String>) -> i32 {
    sentences
        .iter()
        .map(|sentence| sentence.split(' ').collect::<Vec<&str>>().len())
        .max()
        .expect("Parameter 'sentences' is not empty")
        .try_into()
        .expect("Length of sentences do not exceed i32 range")
}
