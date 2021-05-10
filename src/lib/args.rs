use clap::{App, Arg};
use std::str::FromStr;

pub enum Operation {
    COMPRESS,
    DECOMPRESS,
}

pub struct Args {
    pub file: String,
    pub output: String,
    pub operation: Operation,
    pub dict_size: u32,
    pub is_verbose: bool,
}

// Argument names
const FILE_ARG: &str = "file";
const OUTPUT_ARG: &str = "output";
const COMPRESS_ARG: &str = "compress";
const DECOMPRESS_ARG: &str = "decompress";
const DICT_SIZE_ARG: &str = "dictionary_size";
const VERBOSE_ARG: &str = "verbose";

// Default arguments
const DEFAULT_DICT_SIZE: u32 = 256;

pub fn parse_args() -> Result<Args, String> {
    let args = App::new("Rust Compressor")
        .version("0.1.0")
        .author("Severn Everett")
        .about("A utility program for compressing and decompressing files")
        .arg(Arg::with_name(FILE_ARG)
            .short("f")
            .long(FILE_ARG)
            .help("The file to be compressed/decompressed")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name(OUTPUT_ARG)
            .short("o")
            .long(OUTPUT_ARG)
            .help("The file to which the output will be written")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name(COMPRESS_ARG)
            .short("c")
            .help("Conduct compression operation")
        )
        .arg(Arg::with_name(DECOMPRESS_ARG)
            .short("d")
            .help("Conduct decompression operation")
            .conflicts_with(COMPRESS_ARG)
        )
        .arg(Arg::with_name(DICT_SIZE_ARG)
            .short("s")
            .long(DICT_SIZE_ARG)
            .help("The size of the dictionary")
            .takes_value(true))
        .arg(Arg::with_name(VERBOSE_ARG)
            .short("v")
            .long(VERBOSE_ARG)
            .help("Enable verbose printout")
            .takes_value(true))
        .get_matches();
    let file = String::from(args.value_of(FILE_ARG).unwrap());
    let output = String::from(args.value_of(OUTPUT_ARG).unwrap());
    let operation = if args.is_present(COMPRESS_ARG) {
        Operation::COMPRESS
    } else if args.is_present(DECOMPRESS_ARG) {
        Operation::DECOMPRESS
    } else {
        return Err("Either compression or decompression must be selected".to_string());
    };
    let dict_size = match args.value_of(DICT_SIZE_ARG) {
        Some(raw_size) => match u32::from_str(raw_size) {
            Ok(size) => size,
            Err(e) => return Err(e.to_string())
        },
        None => DEFAULT_DICT_SIZE
    };
    let is_verbose = args.value_of(VERBOSE_ARG).is_some();

    return Ok(Args { file, output, operation, dict_size, is_verbose });
}
