#[macro_use]
extern crate clap;
extern crate failure;
extern crate image;

use clap::{App, Arg, ArgMatches};
use failure::Error;
use image::imageops::FilterType;
use image::ImageOutputFormat;
use std::fs::File;
use std::io;
use std::process;
use std::u32;

fn run(matches: &ArgMatches) -> Result<(), Error> {
    let width = value_t!(matches, "width", u32).unwrap_or(u32::MAX);
    let height = value_t!(matches, "height", u32).unwrap_or(u32::MAX);
    let input = matches.value_of_os("INPUT").expect("Missing input file");
    let output = matches.value_of_os("output").expect("Missing output file");
    let img = image::open(&input)?.resize(width, height, FilterType::Lanczos3);
    let format = ImageOutputFormat::Jpeg(80);
    if output == "-" {
        let mut w = io::stdout();
        img.write_to(&mut w, format)?;
    } else {
        let mut w = File::create(output)?;
        img.write_to(&mut w, format)?;
    }
    Ok(())
}

fn main() {
    let matches = App::new("gbhwdb-photo")
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("OUTPUT")
                .required(true)
                .help("Output file, or - to use standard output"),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Input file, or - to use standard input")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("width")
                .short("w")
                .long("width")
                .value_name("WIDTH")
                .required_unless("height"),
        )
        .arg(
            Arg::with_name("height")
                .short("h")
                .long("height")
                .value_name("HEIGHT")
                .required_unless("width"),
        )
        .get_matches();
    if let Err(ref e) = run(&matches) {
        eprintln!("{}\n{}", e, e.backtrace());
        process::exit(1);
    }
}
