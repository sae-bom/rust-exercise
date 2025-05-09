#[allow(dead_code)]
pub fn decompress_rl_elist(nums: &[i32]) -> Vec<i32> {
    nums.chunks_exact(2)
        .flat_map(|pair| {
            std::iter::repeat_n(
                pair[1],
                pair[0]
                    .try_into()
                    .expect("Number must be between 1 and 100"),
            )
        })
        .collect()
}
