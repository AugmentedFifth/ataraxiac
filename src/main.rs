#[macro_use]
extern crate clap;
#[macro_use]
extern crate failure;
extern crate rug;

mod ast;
mod ataraxia;

use ataraxia::ProgParser;
use clap::{App, Arg};
use failure::Error;
use std::{fs::File, io::{BufReader, prelude::*}};


fn main_() -> Result<(), Error> {
    let m = App::new("ataraxiac")
        .author("Zach Comito <zcomito@gmail.com>")
        .about("A compiler for the ataraxia language.")
        .long_about("A compiler for the ataraxia language.\n\n\

                     ataraxiac emits bytecode that can then be interpreted, \
                     or even used to generate native code through an \
                     LLVM backend.\n\n\

                     ataraxia is a high-level, statically typed, \
                     stack-oriented programming language: similar to Joy, \
                     Forth, or PostScript, but more similar to Rust in terms \
                     of typechecking and handling of data.")
        .version(crate_version!())
        .arg(Arg::with_name("infile")
             .index(1)
             .required(true))
        .arg(Arg::with_name("outfile")
             .short("o")
             .long("outfile")
             .alias("out")
             .takes_value(true))
        .arg(Arg::with_name("syntax-only")
             .short("s")
             .long("syntax-only"))
        .get_matches();

    let infile_name = m.value_of("infile").unwrap();

    let infile = File::open(infile_name)?;
    let mut buf_reader = BufReader::new(infile);
    let mut infile_contents = String::new();
    buf_reader.read_to_string(&mut infile_contents)?;

    let prog = ProgParser::new()
        .parse(&infile_contents)
        .map_err(|e| format_err!("{}", e))?;

    println!("{:?}", prog);

    Ok(())
}

fn main() {
    if let Err(e) = main_() {
        e.causes().for_each(|c| eprintln!("{}", c));

        std::process::exit(1);
    }
}
