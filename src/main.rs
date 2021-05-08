mod lib;

use crate::lib::compress;
use std::process::exit;

fn main() {
    let test_str = "TOBEORNOTTOBEORTOBEORNOT";
    // let test_str = "नमस्ते। मैं राज हूँ। मैं अमेरिका से हूँ।";
    let test_comp = match compress::compress(test_str) {
        Ok(comp) => comp,
        Err(reason) => {
            eprintln!("Unable to compress target string. Reason: {}", reason);
            exit(1);
        }
    };
    println!("Compression Results: {} compressed to {}", test_str.len(), test_comp.len());
    println!("Compressed Contents:");
    for i in test_comp {
        print!("{} ", i);
    }
    println!();
}
