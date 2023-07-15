use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}
//---------------------------------------------------------------------
fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;
    let mut line = String::new();
    loop {
        let bytes_read = file.read_line(&mut line)?;
        if bytes_read == 0 {
            break;
        }
        num_lines +=1;
        num_bytes += bytes_read;
        num_words += line.split_whitespace().count();
        num_chars += line.chars().count();
        line.clear();
    }
    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}
//------------------------------------------------------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wcr")
        .version("0.1.0")
        .author("sangkhuudev")
        .about("Rust wc")
        .arg(
            Arg::with_name("lines")
                .short("l")
                .long("lines")
                .value_name("LINES")
                .help("Show number of lines")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("words")
                .short("w")
                .long("words")
                .value_name("WORDS")
                .help("Show number of words")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .value_name("BYTES")
                .help("Show number of bytes")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("chars")
                .short("m")
                .long("chars")
                .value_name("CHARS")
                .help("Show number of characters")
                .takes_value(false)
                .conflicts_with("bytes"),
        )
        .arg(
            Arg::with_name("files")
                .multiple(true)
                .value_name("FILES")
                .help("Input file(s)")
                .default_value("-"),
        )
        .get_matches();

    let mut lines = matches.is_present("lines");
    let mut words = matches.is_present("words");
    let mut bytes = matches.is_present("bytes");
    let chars = matches.is_present("chars");

    if [lines, words, bytes, chars].iter().all(|v| v == &false) {
        lines = true;
        words = true;
        bytes = true;
    }

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines,
        words,
        bytes,
        chars,
    })
}
pub fn run(config: Config) -> MyResult<()> {
    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_bytes = 0;
    let mut total_chars = 0;

    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                if let Ok(info) = count(file) {
                    println!(
                        "{}{}{}{}{}",
                        format_field(info.num_lines, config.lines),
                        format_field(info.num_words, config.words),
                        format_field(info.num_bytes, config.bytes),
                        format_field(info.num_chars, config.chars),
                        if filename == "-" {
                            "".to_string()
                        } else {
                            format!(" {}", &filename)
                        },
                    );

                    total_lines += info.num_lines;
                    total_words += info.num_words;
                    total_bytes += info.num_bytes;
                    total_chars += info.num_chars;
                }
            }
        }
    }

    if config.files.len() > 1 {
        println!(
            "{}{}{}{} total",
            format_field(total_lines, config.lines),
            format_field(total_words, config.words),
            format_field(total_bytes, config.bytes),
            format_field(total_chars, config.chars)
        );
    }

    Ok(())
}

//------------------------------------------------------------------------------
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

//------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::{count, FileInfo ,format_field};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_bytes: 48,
            num_chars: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }

    #[test]
    fn test_format_field() {
        assert_eq!(format_field(1, false), "");
        assert_eq!(format_field(3, true),"       3");
        assert_eq!(format_field(10, true), "      10");
    }
}
//------------------------------------------------------------------------------------------------
fn format_field(value: usize, show: bool) -> String {
    if show {
        format!("{:>8}", value)
    } else {
        "".to_string()
    }
}