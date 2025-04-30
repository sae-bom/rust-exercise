pub fn restore_string(s: String, indices: Vec<i32>) -> String {
    let mut index_char_pair: Vec<(i32, char)> = std::iter::zip(indices, s.chars()).collect();
    index_char_pair.sort_by_key(|&(i, _)| i);
    String::from_iter::<Vec<char>>(index_char_pair.iter().map(|&(_, c)| c).collect())
}
