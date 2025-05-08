use std::collections::{HashMap, hash_map::Entry};

pub fn decode_message(key: String, message: String) -> String {
    let mut convert_map: HashMap<char, char> = HashMap::new();
    convert_map.insert(' ', ' ');

    let _ = key
        .chars()
        .filter(|&c| c != ' ')
        .try_fold(0, |ctr: u32, c| {
            if ctr > 25 {
                Err(ctr)
            } else {
                match convert_map.entry(c) {
                    Entry::Occupied(_) => Ok(ctr),
                    Entry::Vacant(_) => {
                        convert_map.insert(c, char::from_u32(ctr + u32::from('a')).unwrap());
                        Ok(ctr + 1)
                    }
                }
            }
        });
    message
        .chars()
        .map(|c| convert_map.get(&c).unwrap())
        .collect()
}
