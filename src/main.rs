mod lib;

use crate::lib::compress;
use std::process::exit;
use crate::lib::decompress;

fn main() {
    let dict_size = 256;
    let test_str = "TOBEORNOTTOBEORTOBEORNOT";
    // let test_str = "नमस्ते। मैं सेवर्न हूँ। मैं अमेरिका से हूँ।";
    let test_comp = match compress::compress(test_str, &dict_size) {
        Ok(comp) => comp,
        Err(reason) => {
            eprintln!("Unable to compress target. Reason: {}", reason);
            exit(1);
        }
    };
    println!("Compression Results: {} compressed to {}", test_str.len(), test_comp.len());
    println!("Compressed Contents:");
    for i in &test_comp {
        print!("{} ", *i);
    }
    println!();
    let test_decomp = match decompress::decompress(&test_comp, &dict_size) {
        Ok(decomp) => decomp,
        Err(reason) => {
            eprintln!("Unable to decompress target. Reason: {}", reason);
            exit(1);
        }
    };
    println!("Decompressed Contents: {}", test_decomp);
}
