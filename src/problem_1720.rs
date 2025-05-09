#[allow(dead_code)]
pub fn decode(encoded: &[i32], first: i32) -> Vec<i32> {
    std::iter::once(first)
        .chain(encoded.iter().scan(first, |state, &x| {
            *state ^= x;
            Some(*state)
        }))
        .collect()
}
