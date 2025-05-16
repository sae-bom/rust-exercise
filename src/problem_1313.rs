#[allow(dead_code)]
pub fn decompress_rl_elist(nums: &[i32]) -> Result<Vec<i32>, String> {
    Ok(nums
        .chunks_exact(2)
        .map(|pair| {
            let val = *pair
                .get(1)
                .expect("chunks_exact(2) returns a slice with exactly 2 elements.");
            let freq: usize = (*pair
                .first()
                .expect("chunks_exact(2) returns a slice with exactly 2 elements."))
            .try_into()
            .map_err(|_| "Invalid frequency")?;

            Ok(std::iter::repeat_n(val, freq))
        })
        .collect::<Result<Vec<_>, String>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<i32>>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = decompress_rl_elist(&[1, 2, 3, 4]);
        assert_eq!(result, Ok(vec![2, 4, 4, 4]));
    }
}
