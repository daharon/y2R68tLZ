use std::fs::File;
use std::io;
use std::io::BufRead;

use clap::{Arg, crate_name, crate_version};
use rayon::prelude::*;

fn main() {
    // Command-line parameters.
    let matches = clap::App::new(crate_name!())
        .about("Parallel Hashing")
        .version(crate_version!())
        .author("Dan Aharon <dan@aharon.dev>")
        .arg(Arg::with_name("concurrency")
            .short("c")
            .long("concurrency")
            .takes_value(true)
            .value_name("NUM")
            .default_value("1"))
        .arg(Arg::with_name("FILE")
            .help("Text file(s) containing URLs separated by newlines")
            .required(true)
            .multiple(true))
        .get_matches();
    let concurrency = matches.value_of("concurrency").unwrap().parse::<i32>()
        .expect("Failed to parse to integer.");
    let input_files: Vec<_> = matches.values_of("FILE").unwrap().collect();

    eprintln!("Concurrency:  {}", concurrency);
    eprintln!("Input Files:  {:?}", input_files);

    // Read the lines of the input files into a list of strings.
    let urls: Vec<String> = input_files.iter()
        .map(|file_name| {
            match File::open(file_name) {
                Err(err) => {
                    eprintln!("Failed to open {}:  {}", file_name, err);
                    vec![]
                },
                Ok(f) => {
                    let f = io::BufReader::new(f);
                    f.lines()
                        .flatten() // Drop the unsuccessfully read lines.
                        .filter(|s| !s.trim().is_empty()) // Drop the empty lines.
                        .collect()
                }
            }
        })
        .flatten()
        .collect();
    eprintln!("URLs:  {:?}", urls);

    // Using a thread-based parallel iterator provided by the Rayon library.
    // https://crates.io/crates/rayon
    urls.par_iter()
        .map(|url| {
            match reqwest::blocking::get(url) {
                Err(err) => {
                    eprintln!("Failed to download contents of {}:  {}", url, err);
                    "ERROR".to_string()
                },
                Ok(response) => match response.bytes() {
                    Ok(bytes) => format!("{:x}", md5::compute(bytes)).to_string(),
                    Err(err) => {
                        eprintln!("Failed to read contents of {}:  {}", url, err);
                        "ERROR".to_string()
                    }
                },
            }
        })
        // Collect before starting the print iteration. This ensures that the order is maintained.
        .collect::<Vec<String>>()
        .iter()
        // Print each hash to stdout.
        .for_each(|hash| println!("{}", hash));
}
