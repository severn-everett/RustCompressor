use std::collections::HashMap;

pub fn decompress(compressed: &Vec<u32>, dict_size: &u32) -> Result<String, String> {
    if compressed.is_empty() {
        return Err("Nothing to decompress".to_string());
    }
    let mut compressed: Vec<u32> = compressed.to_vec();
    let mut dict_size = *dict_size;
    let mut dictionary = HashMap::new();
    for i in 0..dict_size {
        let c = match char::from_u32(i) {
            Some(value) => value,
            None => return Err(format!("Dictionary size '{}' is too large.", dict_size))
        };
        dictionary.insert(i, c.to_string());
    }

    let first_char = compressed.remove(0);
    let mut w = match char::from_u32(first_char) {
        Some(value) => value.to_string(),
        None => return Err(format!("Bad compressed character: {}", first_char))
    };
    let mut result = String::from(&w);
    for k in compressed {
        let entry = match dictionary.get(&k) {
            Some(value) => String::from(value),
            None => {
                if k == dict_size {
                    let mut entry_val = String::from(&w);
                    entry_val.push(w.chars().nth(0).unwrap());
                    entry_val
                } else {
                    return Err(format!("Bad compressed character: {}", k))
                }
            }
        };
        result.push_str(entry.as_str());
        let mut new_dict_entry = String::from(&w);
        new_dict_entry.push(entry.chars().nth(0).unwrap());
        dictionary.insert(dict_size, new_dict_entry);
        dict_size += 1;

        w = String::from(entry);
    }
    Ok(result)
}