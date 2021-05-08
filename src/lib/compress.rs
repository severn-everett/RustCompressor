use std::collections::HashMap;

pub fn compress(uncompressed: &str, dict_size: &u32) -> Result<Vec<u32>, String> {
    let mut dict_size: u32 = *dict_size;
    let mut dictionary = HashMap::new();
    for i in 0..dict_size {
        let c = match char::from_u32(i) {
            Some(value) => value,
            None => return Err(format!("Dictionary size '{}' is too large.", dict_size))
        };
        dictionary.insert(c.to_string(), i);
    }
    let mut result: Vec<u32> = Vec::new();
    let mut word = String::new();
    for c in uncompressed.chars() {
        let mut wc = String::from(&word);
        wc.push(c);
        if dictionary.contains_key(&wc) {
            word = wc;
        } else {
            let dict_value = match dictionary.get(&word) {
                Some(value) => value,
                None => return Err("Cannot find word value - try increasing dictionary size.".to_string())
            };
            result.push(*dict_value);
            dictionary.insert(wc, dict_size);
            dict_size += 1;
            word = c.to_string();
        }
    }

    if !word.eq("") {
        result.push(*dictionary.get(&word).unwrap());
    }
    Ok(result)
}