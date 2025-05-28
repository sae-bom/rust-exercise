#[allow(dead_code)]
pub fn restore_string(s: &str, indices: Vec<i32>) -> String {
    let mut index_char_pair: Vec<(i32, char)> = std::iter::zip(indices, s.chars()).collect();
    index_char_pair.sort_by_key(|&(i, _)| i);
    index_char_pair.iter().map(|&(_, c)| c).collect()
}
