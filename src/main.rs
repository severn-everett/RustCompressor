use std::process::exit;

use crate::lib::args;
use crate::lib::args::Operation;
use crate::lib::compress;
use crate::lib::decompress;

mod lib;

fn main() {
    let args = match args::parse_args() {
        Ok(a) => a,
        Err(reason) => {
            eprintln!("Error while parsing command args: {}", reason);
            exit(1);
        }
    };
    let _output_file = match args.operation {
        Operation::COMPRESS => {
            match compress::compress(
                &args.file,
                &args.output,
                &args.dict_size,
                &args.is_verbose,
            ) {
                Ok(file) => file,
                Err(reason) => {
                    eprintln!("Unable to compress target: {}", reason);
                    exit(1);
                }
            }
        }
        Operation::DECOMPRESS => {
            match decompress::decompress(
                &args.file,
                &args.output,
                &args.dict_size,
                &args.is_verbose,
            ) {
                Ok(file) => file,
                Err(reason) => {
                    eprintln!("Unable to decompress target: {}", reason);
                    exit(1);
                }
            }
        }
    };
}
