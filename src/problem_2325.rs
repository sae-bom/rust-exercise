use std::collections::{HashMap, hash_map::Entry};

#[allow(dead_code)]
pub fn decode_message(key: &str, message: &str) -> String {
    let mut convert_map: HashMap<char, char> = HashMap::new();

    let _ = key
        .chars()
        .filter(|&c| c.is_ascii_lowercase())
        .try_fold(0, |ctr: u32, c| {
            if ctr > 25 {
                Err(ctr)
            } else {
                match convert_map.entry(c) {
                    Entry::Occupied(_) => Ok(ctr),
                    Entry::Vacant(_) => {
                        let decode_char = char::from_u32(ctr + u32::from('a')).ok_or(ctr)?;
                        convert_map.insert(c, decode_char);
                        Ok(ctr + 1)
                    }
                }
            }
        });
    message
        .chars()
        .map(|c| *convert_map.get(&c).unwrap_or(&c))
        .collect()
}
