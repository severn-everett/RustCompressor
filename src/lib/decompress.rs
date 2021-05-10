use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};

pub fn decompress(input_file: &str, output_file: &str, dict_size: &u32, is_verbose: &bool) -> Result<File, String> {
    let mut input = match File::open(input_file) {
        Ok(f) => f,
        Err(reason) => return Err(reason.to_string())
    };
    let mut read_buffer = [0; 4];
    let mut compressed = Vec::new();
    loop {
        let bytes_read = match input.read(&mut read_buffer) {
            Ok(br) => br,
            Err(reason) => return Err(reason.to_string())
        };
        match bytes_read {
            4 => {
                compressed.push(u32::from_be_bytes(read_buffer));
            }
            0 => break,
            _ => return Err("File has been corrupted.".to_string())
        };
    }

    let decompressed = run_decompression(&compressed, dict_size)?;
    if *is_verbose {
        println!("Decompression Results: {} decompressed to {}", compressed.len(), decompressed.len());
    }

    let mut out_file = match File::create(output_file) {
        Ok(out_file) => out_file,
        Err(reason) => return Err(reason.to_string())
    };
    match out_file.write_all(decompressed.as_bytes()) {
        Ok(_) => Ok(out_file),
        Err(reason) => Err(reason.to_string())
    }
}

fn run_decompression(compressed: &Vec<u32>, dict_size: &u32) -> Result<String, String> {
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
                    return Err(format!("Bad compressed character: {}", k));
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