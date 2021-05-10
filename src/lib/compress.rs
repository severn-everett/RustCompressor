use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;

pub fn compress(input_file: &str, output_file: &str, dict_size: &u32, is_verbose: &bool) -> Result<File, String> {
    let uncompressed = match fs::read_to_string(input_file) {
        Ok(contents) => contents,
        Err(reason) => return Err(reason.to_string())
    };
    let compressed = run_compression(uncompressed.as_str(), dict_size)?;
    if *is_verbose {
        println!("Compression Results: {} compressed to {}", uncompressed.len(), compressed.len());
    }

    let mut compressed_bytes = Vec::with_capacity(4 * compressed.len());
    for c in compressed {
        compressed_bytes.extend(&c.to_be_bytes())
    }
    let mut out_file = match File::create(output_file) {
        Ok(out_file) => out_file,
        Err(reason) => return Err(reason.to_string())
    };
    match out_file.write_all(&compressed_bytes) {
        Ok(_) => Ok(out_file),
        Err(reason) => Err(reason.to_string())
    }
}

fn run_compression(uncompressed: &str, dict_size: &u32) -> Result<Vec<u32>, String> {
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