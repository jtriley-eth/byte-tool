#![feature(iterator_try_collect, iter_intersperse)]

use std::str;

use clap::Parser;
use clap::Subcommand;
use clap::ValueEnum;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, long_help = "input format")]
    input: System,
    #[arg(short, long, long_help = "output format")]
    output: System,
    #[arg(
        short,
        long,
        long_help = "separator string: ', ' = 'xx, xx'",
        requires("grouping")
    )]
    separator: Option<String>,
    #[arg(
        short,
        long,
        value_parser = clap::value_parser!(usize),
        long_help = "byte grouping count: 2 = 'xx xx'",
        requires("separator")
    )]
    grouping: Option<usize>,
    data: String,
}

/// Formatting System for Encoding / Decoding
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Subcommand, Debug)]
pub enum System {
    /// ASCII
    Ascii,
    /// UTF-8
    Utf8,
    /// Hexadecimal
    Hex,
    /// Decimal
    Dec,
}

impl System {
    /// Decodes a string given some System instance
    pub fn decode(&self, data: String) -> Result<Vec<u8>, AppError> {
        match self {
            System::Ascii => {
                let bytes = data.into_bytes();

                match bytes.iter().any(|&byte| byte > 127) {
                    true => Err(AppError::AsciiDecode),
                    false => Ok(bytes),
                }
            }
            System::Utf8 => Ok(data.into_bytes()),
            System::Hex => hex::decode(data.replace("0x", "")).map_err(AppError::HexDecode),
            System::Dec => data
                .split(|c: char| c.is_whitespace() || c == ',')
                .map(|s| u8::from_str_radix(s, 10))
                .map(Result::ok)
                .try_collect()
                .ok_or(AppError::DecDecode),
        }
    }

    /// Encodes a string given some System instance and optionally a separator and grouping pair
    pub fn encode(&self, data: &[u8]) -> Result<String, AppError> {
        let encoded = match self {
            System::Ascii => match data.iter().any(|&byte| byte > 127) {
                true => Err(AppError::AsciiEncode)?,
                false => str::from_utf8(&data)
                    .expect("idt this is fallible")
                    .to_string(),
            },
            System::Utf8 => str::from_utf8(&data)
                .map_err(AppError::Utf8Encode)?
                .to_string(),
            System::Hex => hex::encode(data),
            System::Dec => data.iter().map(u8::to_string).collect(),
        };

        Ok(encoded)
    }

    pub fn interleave(
        &self,
        data: &[u8],
        separator: String,
        grouping: usize,
    ) -> Result<String, AppError> {
        let interleaved = data
            .chunks(grouping)
            .map(|group| self.encode(group))
            .intersperse(Ok(separator))
            .collect::<Result<Vec<String>, AppError>>()?
            .concat();

        Ok(interleaved)
    }
}

/// Error within this application
#[derive(Debug, Clone)]
pub enum AppError {
    /// Decoding Ascii error, invalid value.
    AsciiDecode,
    /// Decoding error from the `hex` crate.
    HexDecode(hex::FromHexError),
    /// Decoding error from the standard library's `u8` type.
    DecDecode,
    /// Encoding Ascii error, invalid value.
    AsciiEncode,
    /// Encoding error from the standar library's `str` type.
    Utf8Encode(str::Utf8Error),
}

fn main() {
    let cli = Cli::parse();

    let decoded = match cli.input.decode(cli.data) {
        Ok(string) => string,
        Err(e) => panic!("Decoding Error: {:#?}", e),
    };

    let encoded = match (cli.grouping, cli.separator) {
        (None, None) => cli.output.encode(&decoded),
        (Some(grouping), Some(separator)) => cli.output.interleave(&decoded, separator, grouping),
        _ => unreachable!(),
    };

    match encoded {
        Ok(string) => println!("{string}"),
        Err(err) => panic!("Encoding Error: {:#?}", err),
    };
}
